use crate::{
	app::Environment,
	components::{
		visibility_blocking, CommandBlocking, CommandInfo, Component,
		DrawableComponent, EventState, InputType, ScrollType,
		TextInputComponent, VerticalScroll,
	},
	keys::{key_match, SharedKeyConfig},
	queue::{InternalEvent, Queue},
	strings,
	ui::style::SharedTheme,
};
use anyhow::Result;
use asyncgit::{
	asyncjob::AsyncSingleJob,
	sync::{self, get_branches_info, BranchInfo, RepoPathRef},
	AsyncCreatePrJob, AsyncGitNotification, CreatePrRequest,
};
use crossterm::event::Event;
use ratatui::{
	layout::{Constraint, Direction, Layout, Rect},
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem, Paragraph},
	Frame,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Focus {
	Branches,
	Title,
	Body,
}

/// Tab to create a GitHub pull request from a local branch via `gh`.
pub struct CreatePrTab {
	repo: RepoPathRef,
	queue: Queue,
	theme: SharedTheme,
	key_config: SharedKeyConfig,
	visible: bool,
	branches: Vec<BranchInfo>,
	selection: usize,
	scroll: VerticalScroll,
	focus: Focus,
	base_branch: String,
	head_branch: String,
	title: TextInputComponent,
	body: TextInputComponent,
	job: AsyncSingleJob<AsyncCreatePrJob>,
	status: String,
	pending: bool,
}

impl CreatePrTab {
	///
	pub fn new(env: &Environment) -> Self {
		let mut title = TextInputComponent::new(
			env,
			&strings::create_pr_title_input(),
			"",
			false,
		)
		.with_input_type(InputType::Singleline);
		title.embed();

		let mut body = TextInputComponent::new(
			env,
			&strings::create_pr_body_input(),
			"",
			false,
		)
		.with_input_type(InputType::Multiline);
		body.embed();

		Self {
			repo: env.repo.clone(),
			queue: env.queue.clone(),
			theme: env.theme.clone(),
			key_config: env.key_config.clone(),
			visible: false,
			branches: Vec::new(),
			selection: 0,
			scroll: VerticalScroll::new(),
			focus: Focus::Branches,
			base_branch: String::new(),
			head_branch: String::new(),
			title,
			body,
			job: AsyncSingleJob::new(env.sender_git.clone()),
			status: String::from(
				"Select a branch, set base/title, then create",
			),
			pending: false,
		}
	}

	///
	pub fn update(&mut self) -> Result<()> {
		if !self.is_visible() {
			return Ok(());
		}

		self.branches = get_branches_info(&self.repo.borrow(), true)?;
		if self.selection >= self.branches.len() {
			self.selection =
				self.branches.len().saturating_sub(1);
		}

		if self.base_branch.is_empty() {
			self.base_branch = default_base_branch(&self.branches);
		}

		if self.head_branch.is_empty() {
			if let Ok(head) =
				sync::get_head_tuple(&self.repo.borrow())
			{
				self.head_branch = head
					.name
					.strip_prefix("refs/heads/")
					.unwrap_or(head.name.as_str())
					.to_string();
			} else if let Some(b) = self.branches.first() {
				self.head_branch = b.name.clone();
			}
		}

		self.apply_focus();
		Ok(())
	}

	///
	pub fn update_git(
		&mut self,
		ev: AsyncGitNotification,
	) -> Result<()> {
		if !self.visible || ev != AsyncGitNotification::CreatePr {
			return Ok(());
		}

		self.pending = false;
		if let Some(job) = self.job.take_last() {
			match job.result() {
				Some(Ok(msg)) => {
					self.status = format!("Created: {msg}");
					self.queue.push(InternalEvent::ShowInfoMsg(msg));
				}
				Some(Err(e)) => {
					let msg = format!("create PR failed:\n{e}");
					self.status = msg.clone();
					self.queue
						.push(InternalEvent::ShowErrorMsg(msg));
				}
				None => {
					self.status =
						String::from("create PR finished with no result");
				}
			}
		}

		Ok(())
	}

	fn apply_focus(&mut self) {
		match self.focus {
			Focus::Branches => {
				self.title.enabled(false);
				self.body.enabled(false);
			}
			Focus::Title => {
				self.title.enabled(true);
				self.body.enabled(false);
			}
			Focus::Body => {
				self.title.enabled(false);
				self.body.enabled(true);
			}
		}
	}

	fn selected_branch(&self) -> Option<&BranchInfo> {
		self.branches.get(self.selection)
	}

	fn move_selection(&mut self, scroll: ScrollType) {
		if self.branches.is_empty() {
			return;
		}
		let max = self.branches.len().saturating_sub(1);
		self.selection = match scroll {
			ScrollType::Up => self.selection.saturating_sub(1),
			ScrollType::Down => (self.selection + 1).min(max),
			ScrollType::Home => 0,
			ScrollType::End => max,
			ScrollType::PageUp => self.selection.saturating_sub(10),
			ScrollType::PageDown => (self.selection + 10).min(max),
		};
	}

	fn set_head_from_selection(&mut self) {
		if let Some(name) =
			self.selected_branch().map(|b| b.name.clone())
		{
			self.head_branch = name.clone();
			if self.title.get_text().trim().is_empty() {
				self.title.set_text(format!("Merge {name}"));
			}
			self.status = format!("Head set to `{name}`");
		}
	}

	fn set_base_from_selection(&mut self) {
		if let Some(name) =
			self.selected_branch().map(|b| b.name.clone())
		{
			self.base_branch = name.clone();
			self.status = format!("Base set to `{name}`");
		}
	}

	fn cycle_focus(&mut self, reverse: bool) {
		self.focus = if reverse {
			match self.focus {
				Focus::Branches => Focus::Body,
				Focus::Title => Focus::Branches,
				Focus::Body => Focus::Title,
			}
		} else {
			match self.focus {
				Focus::Branches => Focus::Title,
				Focus::Title => Focus::Body,
				Focus::Body => Focus::Branches,
			}
		};
		self.apply_focus();
	}

	fn create_pr(&mut self) {
		if self.pending {
			self.status =
				String::from("create PR already in progress…");
			return;
		}

		let title = self.title.get_text().to_string();
		let body = self.body.get_text().to_string();
		if title.trim().is_empty() {
			self.status =
				String::from("title is required to create a PR");
			self.focus = Focus::Title;
			self.apply_focus();
			return;
		}
		if self.head_branch.is_empty() || self.base_branch.is_empty()
		{
			self.status = String::from(
				"base and head branches are required",
			);
			return;
		}

		self.pending = true;
		self.status = format!(
			"Creating PR: {} ← {} …",
			self.base_branch, self.head_branch
		);

		let job = AsyncCreatePrJob::new(
			self.repo.borrow().clone(),
			CreatePrRequest {
				base: self.base_branch.clone(),
				head: self.head_branch.clone(),
				title,
				body,
			},
		);
		self.job.spawn(job);
	}

	fn draw_branches(&self, f: &mut Frame, area: Rect) {
		let height = area.height.saturating_sub(2) as usize;
		self.scroll.update(
			self.selection,
			self.branches.len(),
			height,
		);

		let items: Vec<ListItem> = self
			.branches
			.iter()
			.enumerate()
			.skip(self.scroll.get_top())
			.take(height)
			.map(|(idx, b)| {
				let selected = idx == self.selection;
				let is_head = b.name == self.head_branch;
				let is_base = b.name == self.base_branch;
				let marker = if is_head && is_base {
					"*="
				} else if is_head {
					"* "
				} else if is_base {
					"= "
				} else {
					"  "
				};
				let style = self.theme.text(
					true,
					selected && self.focus == Focus::Branches,
				);
				ListItem::new(Line::from(Span::styled(
					format!("{marker}{}", b.name),
					style,
				)))
			})
			.collect();

		let title = if self.focus == Focus::Branches {
			"Branches [focused]"
		} else {
			"Branches"
		};

		f.render_widget(
			List::new(items).block(
				Block::default()
					.title(title)
					.borders(Borders::ALL)
					.border_style(
						self.theme.block(self.focus == Focus::Branches),
					),
			),
			area,
		);
	}

	fn draw_form(&self, f: &mut Frame, area: Rect) -> Result<()> {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(3),
				Constraint::Length(3),
				Constraint::Length(3),
				Constraint::Min(5),
				Constraint::Length(3),
			])
			.split(area);

		let summary = Paragraph::new(Line::from(vec![
			Span::styled("Base: ", self.theme.text(true, false)),
			Span::styled(
				self.base_branch.clone(),
				self.theme.branch(false, false),
			),
			Span::raw("  ←  "),
			Span::styled("Head: ", self.theme.text(true, false)),
			Span::styled(
				self.head_branch.clone(),
				self.theme.branch(false, true),
			),
		]))
		.block(
			Block::default()
				.title("Pull Request")
				.borders(Borders::ALL),
		);
		f.render_widget(summary, chunks[0]);

		let help = Paragraph::new(Line::from(Span::styled(
			"Enter=set head  |  b=set base  |  Tab=focus  |  Esc=branches  |  c=create",
			self.theme.text(false, false),
		)));
		f.render_widget(help, chunks[1]);

		self.title.draw(f, chunks[2])?;
		self.body.draw(f, chunks[3])?;

		let status = Paragraph::new(Line::from(Span::styled(
			self.status.clone(),
			self.theme.text(true, false),
		)))
		.block(
			Block::default().title("Status").borders(Borders::ALL),
		);
		f.render_widget(status, chunks[4]);

		Ok(())
	}
}

fn default_base_branch(branches: &[BranchInfo]) -> String {
	for name in ["main", "master", "develop"] {
		if branches.iter().any(|b| b.name == name) {
			return name.to_string();
		}
	}
	branches
		.first()
		.map(|b| b.name.clone())
		.unwrap_or_default()
}

impl DrawableComponent for CreatePrTab {
	fn draw(&self, f: &mut Frame, rect: Rect) -> Result<()> {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage(35),
				Constraint::Percentage(65),
			])
			.split(rect);

		self.draw_branches(f, chunks[0]);
		self.draw_form(f, chunks[1])?;
		Ok(())
	}
}

impl Component for CreatePrTab {
	fn commands(
		&self,
		out: &mut Vec<CommandInfo>,
		force_all: bool,
	) -> CommandBlocking {
		if self.visible || force_all {
			out.push(CommandInfo::new(
				strings::commands::create_pr_set_head(
					&self.key_config,
				),
				!self.branches.is_empty(),
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_set_base(
					&self.key_config,
				),
				!self.branches.is_empty(),
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_submit(
					&self.key_config,
				),
				!self.pending,
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_focus(
					&self.key_config,
				),
				true,
				true,
			));
		}
		visibility_blocking(self)
	}

	fn event(&mut self, ev: &Event) -> Result<EventState> {
		if !self.is_visible() {
			return Ok(EventState::NotConsumed);
		}

		// Handle navigation keys before embedded text inputs.
		// TextInputComponent treats Esc as "close popup" (hides itself)
		// and Tab as insert-tab — both break this tab's UX.
		if let Event::Key(k) = ev {
			if key_match(k, self.key_config.keys.exit_popup) {
				if self.focus != Focus::Branches {
					self.focus = Focus::Branches;
					self.apply_focus();
					return Ok(EventState::Consumed);
				}
				return Ok(EventState::NotConsumed);
			}
			if key_match(k, self.key_config.keys.tab_toggle) {
				self.cycle_focus(false);
				return Ok(EventState::Consumed);
			}
			if key_match(k, self.key_config.keys.tab_toggle_reverse)
			{
				self.cycle_focus(true);
				return Ok(EventState::Consumed);
			}
			if key_match(k, self.key_config.keys.move_right)
				&& self.focus == Focus::Branches
			{
				self.cycle_focus(false);
				return Ok(EventState::Consumed);
			}
			if key_match(k, self.key_config.keys.move_left)
				&& self.focus != Focus::Branches
			{
				self.focus = Focus::Branches;
				self.apply_focus();
				return Ok(EventState::Consumed);
			}
		}

		if self.focus == Focus::Title
			&& self.title.event(ev)?.is_consumed()
		{
			return Ok(EventState::Consumed);
		}
		if self.focus == Focus::Body
			&& self.body.event(ev)?.is_consumed()
		{
			return Ok(EventState::Consumed);
		}

		if let Event::Key(k) = ev {
			if self.focus == Focus::Branches {
				if key_match(k, self.key_config.keys.move_up) {
					self.move_selection(ScrollType::Up);
					return Ok(EventState::Consumed);
				}
				if key_match(k, self.key_config.keys.move_down) {
					self.move_selection(ScrollType::Down);
					return Ok(EventState::Consumed);
				}
				if key_match(k, self.key_config.keys.home) {
					self.move_selection(ScrollType::Home);
					return Ok(EventState::Consumed);
				}
				if key_match(k, self.key_config.keys.end) {
					self.move_selection(ScrollType::End);
					return Ok(EventState::Consumed);
				}
				if key_match(k, self.key_config.keys.enter) {
					self.set_head_from_selection();
					return Ok(EventState::Consumed);
				}
			}

			if k.code == crossterm::event::KeyCode::Char('b')
				&& k.modifiers.is_empty()
			{
				self.set_base_from_selection();
				return Ok(EventState::Consumed);
			}

			if key_match(k, self.key_config.keys.open_commit) {
				self.create_pr();
				return Ok(EventState::Consumed);
			}
		}

		Ok(EventState::NotConsumed)
	}

	fn is_visible(&self) -> bool {
		self.visible
	}

	fn hide(&mut self) {
		self.visible = false;
		self.title.hide();
		self.body.hide();
	}

	fn show(&mut self) -> Result<()> {
		self.visible = true;
		self.title.show()?;
		self.body.show()?;
		self.update()?;
		self.apply_focus();
		Ok(())
	}
}
