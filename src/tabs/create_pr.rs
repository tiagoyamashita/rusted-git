use crate::{
	app::Environment,
	components::{
		visibility_blocking, CommandBlocking, CommandInfo,
		CommitDetailsComponent, Component, DiffComponent,
		DrawableComponent, EventState, InputType, ScrollType,
		TextInputComponent, VerticalScroll,
	},
	keys::{key_match, SharedKeyConfig},
	options::SharedOptions,
	queue::{InternalEvent, Queue},
	strings,
	ui::style::SharedTheme,
};
use anyhow::Result;
use asyncgit::{
	asyncjob::AsyncSingleJob,
	sync::{
		self, commit_files::OldNew, get_branches_info, BranchInfo,
		CommitId, RepoPathRef,
	},
	AsyncCreatePrJob, AsyncDiff, AsyncGitNotification,
	AsyncListOpenPrsJob, CommitFilesParams, CreatePrRequest,
	DiffParams, DiffType, OpenPrInfo,
};
use crossterm::event::Event;
use ratatui::{
	layout::{Constraint, Direction, Layout, Margin, Rect},
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem, Paragraph},
	Frame,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Focus {
	Branches,
	Title,
	Body,
	Files,
	Diff,
}

/// Tab to create a GitHub pull request from remote branches via `gh`.
pub struct CreatePrTab {
	repo: RepoPathRef,
	queue: Queue,
	theme: SharedTheme,
	key_config: SharedKeyConfig,
	options: SharedOptions,
	visible: bool,
	branches: Vec<BranchInfo>,
	selection: usize,
	scroll: VerticalScroll,
	focus: Focus,
	base_branch: String,
	head_branch: String,
	title: TextInputComponent,
	body: TextInputComponent,
	details: CommitDetailsComponent,
	diff: DiffComponent,
	git_diff: AsyncDiff,
	job: AsyncSingleJob<AsyncCreatePrJob>,
	open_pr_job: AsyncSingleJob<AsyncListOpenPrsJob>,
	open_prs: Vec<OpenPrInfo>,
	open_prs_requested: bool,
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
			options: env.options.clone(),
			visible: false,
			branches: Vec::new(),
			selection: 0,
			scroll: VerticalScroll::new(),
			focus: Focus::Branches,
			base_branch: String::new(),
			head_branch: String::new(),
			title,
			body,
			details: CommitDetailsComponent::new(env),
			diff: DiffComponent::new(env, true),
			git_diff: AsyncDiff::new(
				env.repo.borrow().clone(),
				&env.sender_git,
			),
			job: AsyncSingleJob::new(env.sender_git.clone()),
			open_pr_job: AsyncSingleJob::new(env.sender_git.clone()),
			open_prs: Vec::new(),
			open_prs_requested: false,
			status: String::from(
				"1) pick branches  2) Tab to PR Title  3) Tab to Description  4) c to create",
			),
			pending: false,
		}
	}

	///
	pub fn update(&mut self) -> Result<()> {
		if !self.is_visible() {
			return Ok(());
		}

		let repo = self.repo.borrow();
		let mut local_branches = get_branches_info(&repo, true)?;
		let remote_branches = get_branches_info(&repo, false)?;

		let current_remote =
			sync::get_head_tuple(&repo).ok().and_then(|head| {
				let local_name = head
					.name
					.strip_prefix("refs/heads/")
					.unwrap_or(head.name.as_str());
				local_branches
					.iter()
					.find(|branch| branch.name == local_name)
					.and_then(BranchInfo::local_details)
					.and_then(|details| details.upstream.as_ref())
					.and_then(|upstream| {
						remote_branches
							.iter()
							.find(|branch| {
								branch.reference == upstream.reference
							})
							.map(|branch| branch.name.clone())
					})
			});

		local_branches.extend(remote_branches);
		self.branches = local_branches;
		drop(repo);
		if self.selection >= self.branches.len() {
			self.selection = self.branches.len().saturating_sub(1);
		}

		if self.base_branch.is_empty() {
			self.base_branch = default_base_branch(&self.branches);
		}

		if self.head_branch.is_empty() {
			self.head_branch = current_remote.unwrap_or_default();
			if self.head_branch.is_empty() {
				self.status = String::from(
					"Current branch has no remote branch. Push it, then select a [remote] head.",
				);
			}
		}
		if !self.open_prs_requested {
			self.open_prs_requested = true;
			self.open_pr_job.spawn(AsyncListOpenPrsJob::new(
				self.repo.borrow().clone(),
			));
		}

		self.refresh_compare()?;
		self.apply_focus();
		Ok(())
	}

	///
	pub fn update_git(
		&mut self,
		ev: AsyncGitNotification,
	) -> Result<()> {
		if !self.visible {
			return Ok(());
		}

		match ev {
			AsyncGitNotification::CreatePr => {
				self.pending = false;
				if let Some(job) = self.job.take_last() {
					match job.result() {
						Some(Ok(msg)) => {
							self.status = format!("Created: {msg}");
							self.open_pr_job.spawn(
								AsyncListOpenPrsJob::new(
									self.repo.borrow().clone(),
								),
							);
							self.queue.push(
								InternalEvent::ShowInfoMsg(msg),
							);
						}
						Some(Err(e)) => {
							let msg =
								format!("create PR failed:\n{e}");
							self.status = msg.clone();
							self.queue.push(
								InternalEvent::ShowErrorMsg(msg),
							);
						}
						None => {
							self.status = String::from(
								"create PR finished with no result",
							);
						}
					}
				}
			}
			AsyncGitNotification::CommitFiles => {
				self.refresh_compare()?;
			}
			AsyncGitNotification::Diff => {
				self.update_diff()?;
			}
			AsyncGitNotification::OpenPrs => {
				if let Some(job) = self.open_pr_job.take_last() {
					match job.result() {
						Some(Ok(open_prs)) => {
							self.open_prs = open_prs;
						}
						Some(Err(error)) => {
							self.status = format!(
								"Could not list open PRs: {error}"
							);
						}
						None => {}
					}
				}
			}
			_ => {}
		}

		Ok(())
	}

	///
	pub fn any_work_pending(&self) -> bool {
		self.git_diff.is_pending()
			|| self.details.any_work_pending()
			|| self.open_pr_job.is_pending()
			|| self.pending
	}

	fn has_compare(&self) -> bool {
		self.compare_ids().is_some()
	}

	fn tip_of(&self, branch: &str) -> Option<CommitId> {
		self.branches
			.iter()
			.find(|b| b.name == branch)
			.map(|b| b.top_commit)
	}

	fn compare_ids(&self) -> Option<OldNew<CommitId>> {
		if self.base_branch.is_empty()
			|| self.head_branch.is_empty()
			|| self.base_branch == self.head_branch
		{
			return None;
		}
		Some(OldNew {
			old: self.tip_of(&self.base_branch)?,
			new: self.tip_of(&self.head_branch)?,
		})
	}

	fn refresh_compare(&mut self) -> Result<()> {
		if let Some(ids) = self.compare_ids() {
			if !self.details.is_visible() {
				self.details.show()?;
			}
			self.details.set_commits(
				Some(CommitFilesParams::from(ids)),
				None,
			)?;
			self.update_diff()?;
		} else {
			self.details.set_commits(None, None)?;
			self.details.hide();
			self.diff.clear(false);
			if matches!(self.focus, Focus::Files | Focus::Diff) {
				self.focus = Focus::Branches;
			}
		}
		Ok(())
	}

	fn update_diff(&mut self) -> Result<()> {
		if let Some(ids) = self.compare_ids() {
			if let Some(f) = self.details.files().selection_file() {
				let diff_params = DiffParams {
					path: f.path.clone(),
					diff_type: DiffType::Commits(ids),
					options: self.options.borrow().diff_options(),
				};

				if let Some((params, last)) = self.git_diff.last()? {
					if params == diff_params {
						self.diff.update(f.path, false, last);
						return Ok(());
					}
				}

				self.git_diff.request(diff_params)?;
				self.diff.clear(true);
				return Ok(());
			}
		}

		self.diff.clear(false);
		Ok(())
	}

	fn apply_focus(&mut self) {
		let title = self.focus == Focus::Title;
		let body = self.focus == Focus::Body;
		let files = self.focus == Focus::Files;
		let diff = self.focus == Focus::Diff;

		self.title.enabled(title);
		self.body.enabled(body);
		self.details.focus(files);
		self.diff.focus(diff);
	}

	fn selected_branch(&self) -> Option<&BranchInfo> {
		self.branches.get(self.selection)
	}

	fn selected_remote_branch(&self) -> Option<&BranchInfo> {
		self.selected_branch().filter(|branch| !branch.is_local())
	}

	fn is_remote_selection(&self, name: &str) -> bool {
		self.branches
			.iter()
			.any(|branch| branch.name == name && !branch.is_local())
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
		if let Some(name) = self
			.selected_remote_branch()
			.map(|branch| branch.name.clone())
		{
			self.head_branch = name.clone();
			if self.title.get_text().trim().is_empty() {
				self.title.set_text(format!("Merge {name}"));
			}
			self.status = if self.has_compare() {
				format!(
					"Head `{name}` vs base `{}` — diff on the right. Tab edits title.",
					self.base_branch
				)
			} else {
				format!(
					"Head set to `{name}`. Edit title, then Esc back and Tab for next tab."
				)
			};
			self.focus = Focus::Title;
			let _ = self.refresh_compare();
			self.apply_focus();
		} else {
			self.status = String::from(
				"PR head must be a [remote] branch. Push the local branch first.",
			);
		}
	}

	fn set_base_from_selection(&mut self) {
		if let Some(name) = self
			.selected_remote_branch()
			.map(|branch| branch.name.clone())
		{
			self.base_branch = name.clone();
			self.status = if self.has_compare() {
				format!(
					"Base `{name}` vs head `{}` — diff on the right.",
					self.head_branch
				)
			} else {
				format!(
					"Base set to `{name}`. Enter sets head; Right edits title."
				)
			};
			let _ = self.refresh_compare();
		} else {
			self.status =
				String::from("PR base must be a [remote] branch.");
		}
	}

	fn cycle_focus(&mut self, reverse: bool) {
		let has_compare = self.has_compare();
		self.focus = if reverse {
			match self.focus {
				Focus::Branches => {
					if has_compare {
						Focus::Diff
					} else {
						Focus::Body
					}
				}
				Focus::Title => Focus::Branches,
				Focus::Body => Focus::Title,
				Focus::Files => Focus::Body,
				Focus::Diff => Focus::Files,
			}
		} else {
			match self.focus {
				Focus::Branches => Focus::Title,
				Focus::Title => Focus::Body,
				Focus::Body => {
					if has_compare {
						Focus::Files
					} else {
						Focus::Branches
					}
				}
				Focus::Files => Focus::Diff,
				Focus::Diff => Focus::Branches,
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
			self.status =
				String::from("base and head branches are required");
			return;
		}
		if !self.is_remote_selection(&self.head_branch)
			|| !self.is_remote_selection(&self.base_branch)
		{
			self.status = String::from(
				"base and head must both be remote branches",
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
				base: remote_branch_name(&self.base_branch)
					.to_string(),
				head: remote_branch_name(&self.head_branch)
					.to_string(),
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
				let kind = if b.is_local() {
					"[local] "
				} else {
					"[remote] "
				};
				let merge_status = branch_merge_status(b);
				let open_pr_status =
					branch_open_pr_status(b, &self.open_prs);
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
					format!(
						"{marker}{kind}{} {merge_status}{open_pr_status}",
						b.name,
					),
					style,
				)))
			})
			.collect();

		let title = if self.focus == Focus::Branches {
			"Branches [focused]  Enter=head  b=base (remote only)"
		} else {
			"Branches"
		};

		f.render_widget(
			List::new(items).block(
				Block::default()
					.title(title)
					.borders(Borders::ALL)
					.border_style(
						self.theme
							.block(self.focus == Focus::Branches),
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
				Constraint::Length(2),
				Constraint::Length(3),
				Constraint::Min(6),
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
				.title("Pull Request target")
				.borders(Borders::ALL),
		);
		f.render_widget(summary, chunks[0]);

		let help_text = if self.has_compare() {
			"Enter=head  |  b=base  |  Tab=next  |  Right→files/diff  |  Esc=back  |  c=create"
		} else {
			"Enter=set head  |  b=set base  |  Right/Tab=edit  |  Esc=branches  |  Tab(on branches)=next tab  |  c=create"
		};
		let help = Paragraph::new(Line::from(Span::styled(
			help_text,
			self.theme.text(false, false),
		)));
		f.render_widget(help, chunks[1]);

		self.draw_labeled_input(
			f,
			chunks[2],
			"PR Title",
			self.focus == Focus::Title,
			&self.title,
		)?;
		self.draw_labeled_input(
			f,
			chunks[3],
			"PR Description",
			self.focus == Focus::Body,
			&self.body,
		)?;

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

	fn draw_compare(&self, f: &mut Frame, area: Rect) -> Result<()> {
		let percentages = if self.diff.focused() {
			(0, 100)
		} else if self.details.focused() {
			(55, 45)
		} else {
			(45, 55)
		};

		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage(percentages.0),
				Constraint::Percentage(percentages.1),
			])
			.split(area);

		self.details.draw(f, chunks[0])?;
		self.diff.draw(f, chunks[1])?;
		Ok(())
	}

	fn draw_labeled_input(
		&self,
		f: &mut Frame,
		area: Rect,
		label: &str,
		focused: bool,
		input: &TextInputComponent,
	) -> Result<()> {
		let title = if focused {
			format!("{label} [focused — type here]")
		} else {
			format!("{label} (Tab to edit)")
		};

		f.render_widget(
			Block::default()
				.title(title)
				.borders(Borders::ALL)
				.border_style(self.theme.block(focused)),
			area,
		);

		let inner = area.inner(Margin {
			horizontal: 1,
			vertical: 1,
		});
		input.draw(f, inner)
	}
}

fn default_base_branch(branches: &[BranchInfo]) -> String {
	for name in ["main", "master", "develop"] {
		if let Some(branch) = branches.iter().find(|branch| {
			!branch.is_local()
				&& remote_branch_name(&branch.name) == name
		}) {
			return branch.name.clone();
		}
	}
	branches
		.iter()
		.find(|branch| !branch.is_local())
		.map(|branch| branch.name.clone())
		.unwrap_or_default()
}

fn remote_branch_name(name: &str) -> &str {
	name.split_once('/').map_or(name, |(_, branch)| branch)
}

fn branch_merge_status(branch: &BranchInfo) -> String {
	if matches!(remote_branch_name(&branch.name), "main" | "master") {
		return String::from("[primary]");
	}
	if let Some(target) = &branch.merged_into {
		return format!("[merged → {}]", remote_branch_name(target));
	}
	if let Some(target) = &branch.merge_target {
		return format!(
			"[not merged → {}]",
			remote_branch_name(target)
		);
	}
	String::from("[no main/master]")
}

fn branch_open_pr_status(
	branch: &BranchInfo,
	open_prs: &[OpenPrInfo],
) -> String {
	if branch.is_local() {
		return String::new();
	}

	open_prs
		.iter()
		.filter(|pr| pr.head == remote_branch_name(&branch.name))
		.map(|pr| format!(" [open PR #{} → {}]", pr.number, pr.base))
		.collect()
}

impl DrawableComponent for CreatePrTab {
	fn draw(&self, f: &mut Frame, rect: Rect) -> Result<()> {
		if self.has_compare() {
			let chunks = Layout::default()
				.direction(Direction::Horizontal)
				.constraints([
					Constraint::Percentage(22),
					Constraint::Percentage(30),
					Constraint::Percentage(48),
				])
				.split(rect);

			self.draw_branches(f, chunks[0]);
			self.draw_form(f, chunks[1])?;
			self.draw_compare(f, chunks[2])?;
		} else {
			let chunks = Layout::default()
				.direction(Direction::Horizontal)
				.constraints([
					Constraint::Percentage(35),
					Constraint::Percentage(65),
				])
				.split(rect);

			self.draw_branches(f, chunks[0]);
			self.draw_form(f, chunks[1])?;
		}
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
			let remote_selected = self
				.selected_branch()
				.is_some_and(|branch| !branch.is_local());
			out.push(CommandInfo::new(
				strings::commands::create_pr_set_head(
					&self.key_config,
				),
				remote_selected,
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_set_base(
					&self.key_config,
				),
				remote_selected,
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_submit(&self.key_config),
				!self.pending
					&& self.is_remote_selection(&self.base_branch)
					&& self.is_remote_selection(&self.head_branch),
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_focus(&self.key_config),
				true,
				true,
			));
			out.push(CommandInfo::new(
				strings::commands::create_pr_back(&self.key_config),
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

		// Esc / Tab / Left / Right before text inputs so Esc does not
		// hide the embedded fields and Tab does not insert a tab char.
		if let Event::Key(k) = ev {
			if key_match(k, self.key_config.keys.exit_popup) {
				match self.focus {
					Focus::Branches => {
						return Ok(EventState::NotConsumed);
					}
					Focus::Diff => {
						self.focus = Focus::Files;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					Focus::Files => {
						self.focus = Focus::Body;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					Focus::Title | Focus::Body => {
						self.focus = Focus::Branches;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
				}
			}
			if key_match(k, self.key_config.keys.tab_toggle) {
				if self.focus == Focus::Branches {
					return Ok(EventState::NotConsumed);
				}
				self.cycle_focus(false);
				return Ok(EventState::Consumed);
			}
			if key_match(k, self.key_config.keys.tab_toggle_reverse) {
				if self.focus == Focus::Branches {
					return Ok(EventState::NotConsumed);
				}
				self.cycle_focus(true);
				return Ok(EventState::Consumed);
			}
			if key_match(k, self.key_config.keys.move_right) {
				match self.focus {
					Focus::Branches => {
						self.cycle_focus(false);
						return Ok(EventState::Consumed);
					}
					Focus::Body if self.has_compare() => {
						self.focus = Focus::Files;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					Focus::Files
						if self
							.details
							.files()
							.selection_file()
							.is_some() =>
					{
						self.focus = Focus::Diff;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					_ => {}
				}
			}
			if key_match(k, self.key_config.keys.move_left) {
				match self.focus {
					Focus::Diff => {
						self.focus = Focus::Files;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					Focus::Files => {
						self.focus = Focus::Body;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					Focus::Title | Focus::Body => {
						self.focus = Focus::Branches;
						self.apply_focus();
						return Ok(EventState::Consumed);
					}
					Focus::Branches => {}
				}
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
		if self.focus == Focus::Files
			&& self.details.event(ev)?.is_consumed()
		{
			self.update_diff()?;
			return Ok(EventState::Consumed);
		}
		if self.focus == Focus::Diff
			&& self.diff.event(ev)?.is_consumed()
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
				&& !matches!(self.focus, Focus::Title | Focus::Body)
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
		self.details.hide();
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

#[cfg(test)]
mod tests {
	use super::remote_branch_name;

	#[test]
	fn remote_branch_name_removes_only_remote_prefix() {
		assert_eq!(remote_branch_name("origin/main"), "main");
		assert_eq!(
			remote_branch_name("upstream/feature/nested"),
			"feature/nested"
		);
		assert_eq!(remote_branch_name("main"), "main");
	}
}
