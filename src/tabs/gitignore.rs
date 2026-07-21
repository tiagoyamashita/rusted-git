use std::{fs, io::ErrorKind, path::PathBuf};

use crate::{
	app::Environment,
	components::{
		visibility_blocking, CommandBlocking, CommandInfo, Component,
		DrawableComponent, EventState, TextInputComponent,
	},
	keys::{key_match, SharedKeyConfig},
	queue::{InternalEvent, Queue},
	strings,
};
use anyhow::{Context, Result};
use asyncgit::sync::{utils::repo_work_dir, RepoPathRef};
use crossterm::event::Event;
use ratatui::{
	layout::{Constraint, Direction, Layout, Rect},
	text::Line,
	widgets::Paragraph,
	Frame,
};

/// Editor for the repository's root `.gitignore` file.
pub struct GitignoreTab {
	repo: RepoPathRef,
	queue: Queue,
	key_config: SharedKeyConfig,
	editor: TextInputComponent,
	visible: bool,
	loaded: bool,
	status: String,
}

impl GitignoreTab {
	/// Create the `.gitignore` editor tab.
	pub fn new(env: &Environment) -> Self {
		let mut editor = TextInputComponent::new(
			env,
			".gitignore",
			"Add one ignore pattern per line",
			false,
		);
		editor.embed();
		editor.enabled(true);

		Self {
			repo: env.repo.clone(),
			queue: env.queue.clone(),
			key_config: env.key_config.clone(),
			editor,
			visible: false,
			loaded: false,
			status: String::from("Ctrl+S: save .gitignore"),
		}
	}

	fn path(&self) -> Result<PathBuf> {
		Ok(PathBuf::from(repo_work_dir(&self.repo.borrow())?)
			.join(".gitignore"))
	}

	fn load(&mut self) -> Result<()> {
		let path = self.path()?;
		let content = match fs::read_to_string(&path) {
			Ok(content) => content,
			Err(error) if error.kind() == ErrorKind::NotFound => {
				String::new()
			}
			Err(error) => {
				return Err(error).with_context(|| {
					format!("failed to read {}", path.display())
				});
			}
		};
		self.editor.set_text(content);
		self.loaded = true;
		self.status = format!("Editing {}", path.display());
		Ok(())
	}

	fn save(&mut self) -> Result<()> {
		let path = self.path()?;
		fs::write(&path, self.editor.get_text()).with_context(
			|| format!("failed to write {}", path.display()),
		)?;
		let message = format!("Saved {}", path.display());
		self.status.clone_from(&message);
		self.queue.push(InternalEvent::ShowInfoMsg(message));
		Ok(())
	}
}

impl DrawableComponent for GitignoreTab {
	fn draw(&self, frame: &mut Frame, area: Rect) -> Result<()> {
		if !self.visible {
			return Ok(());
		}

		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Min(3), Constraint::Length(1)])
			.split(area);
		self.editor.draw(frame, chunks[0])?;
		frame.render_widget(
			Paragraph::new(Line::from(self.status.as_str())),
			chunks[1],
		);
		Ok(())
	}
}

impl Component for GitignoreTab {
	fn commands(
		&self,
		out: &mut Vec<CommandInfo>,
		force_all: bool,
	) -> CommandBlocking {
		out.push(CommandInfo::new(
			strings::commands::gitignore_save(&self.key_config),
			self.visible,
			self.visible || force_all,
		));
		visibility_blocking(self)
	}

	fn event(&mut self, event: &Event) -> Result<EventState> {
		if !self.visible {
			return Ok(EventState::NotConsumed);
		}

		if let Event::Key(key) = event {
			if key_match(key, self.key_config.keys.gitignore_save) {
				if let Err(error) = self.save() {
					let message = format!(
						"Could not save .gitignore:\n{error:#}"
					);
					self.status.clone_from(&message);
					self.queue
						.push(InternalEvent::ShowErrorMsg(message));
				}
				return Ok(EventState::Consumed);
			}

			if key_match(key, self.key_config.keys.tab_toggle)
				|| key_match(
					key,
					self.key_config.keys.tab_toggle_reverse,
				) {
				return Ok(EventState::NotConsumed);
			}
		}

		self.editor.event(event)
	}

	fn is_visible(&self) -> bool {
		self.visible
	}

	fn hide(&mut self) {
		self.visible = false;
		self.editor.hide();
	}

	fn show(&mut self) -> Result<()> {
		self.visible = true;
		if !self.loaded {
			self.load()?;
		}
		self.editor.show()
	}
}
