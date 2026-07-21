use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
use struct_patch::traits::Patch as PatchTrait;
use struct_patch::Patch;

#[derive(Debug, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub struct RustedGitKeyEvent {
	pub code: KeyCode,
	pub modifiers: KeyModifiers,
}

impl RustedGitKeyEvent {
	pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
		Self { code, modifiers }
	}
}

pub fn key_match(ev: &KeyEvent, binding: RustedGitKeyEvent) -> bool {
	ev.code == binding.code && ev.modifiers == binding.modifiers
}

impl PartialEq for RustedGitKeyEvent {
	fn eq(&self, other: &Self) -> bool {
		let ev: KeyEvent = self.into();
		let other: KeyEvent = other.into();
		ev == other
	}
}

impl From<&RustedGitKeyEvent> for KeyEvent {
	fn from(other: &RustedGitKeyEvent) -> Self {
		Self::new(other.code, other.modifiers)
	}
}

#[derive(Debug, Clone, Patch)]
#[patch(attribute(derive(Deserialize, Debug)))]
pub struct KeysList {
	pub tab_status: RustedGitKeyEvent,
	pub tab_log: RustedGitKeyEvent,
	pub tab_files: RustedGitKeyEvent,
	pub tab_stashing: RustedGitKeyEvent,
	pub tab_stashes: RustedGitKeyEvent,
	pub tab_graph: RustedGitKeyEvent,
	pub tab_create_pr: RustedGitKeyEvent,
	pub tab_toggle: RustedGitKeyEvent,
	pub tab_toggle_reverse: RustedGitKeyEvent,
	pub toggle_workarea: RustedGitKeyEvent,
	pub exit: RustedGitKeyEvent,
	pub quit: RustedGitKeyEvent,
	pub exit_popup: RustedGitKeyEvent,
	pub open_commit: RustedGitKeyEvent,
	pub open_commit_editor: RustedGitKeyEvent,
	pub open_help: RustedGitKeyEvent,
	pub open_options: RustedGitKeyEvent,
	pub move_left: RustedGitKeyEvent,
	pub move_right: RustedGitKeyEvent,
	pub move_up: RustedGitKeyEvent,
	pub move_down: RustedGitKeyEvent,
	pub tree_collapse_recursive: RustedGitKeyEvent,
	pub tree_expand_recursive: RustedGitKeyEvent,
	pub home: RustedGitKeyEvent,
	pub end: RustedGitKeyEvent,
	pub popup_up: RustedGitKeyEvent,
	pub popup_down: RustedGitKeyEvent,
	pub page_down: RustedGitKeyEvent,
	pub page_up: RustedGitKeyEvent,
	pub shift_up: RustedGitKeyEvent,
	pub shift_down: RustedGitKeyEvent,
	pub enter: RustedGitKeyEvent,
	pub blame: RustedGitKeyEvent,
	pub file_history: RustedGitKeyEvent,
	pub edit_file: RustedGitKeyEvent,
	pub status_stage_all: RustedGitKeyEvent,
	pub status_reset_item: RustedGitKeyEvent,
	pub status_ignore_file: RustedGitKeyEvent,
	pub diff_stage_lines: RustedGitKeyEvent,
	pub diff_reset_lines: RustedGitKeyEvent,
	pub stashing_save: RustedGitKeyEvent,
	pub stashing_toggle_untracked: RustedGitKeyEvent,
	pub stashing_toggle_index: RustedGitKeyEvent,
	pub stash_apply: RustedGitKeyEvent,
	pub stash_open: RustedGitKeyEvent,
	pub stash_drop: RustedGitKeyEvent,
	pub cmd_bar_toggle: RustedGitKeyEvent,
	pub log_tag_commit: RustedGitKeyEvent,
	pub log_mark_commit: RustedGitKeyEvent,
	pub log_checkout_commit: RustedGitKeyEvent,
	pub log_reset_commit: RustedGitKeyEvent,
	pub log_reword_commit: RustedGitKeyEvent,
	pub log_find: RustedGitKeyEvent,
	pub find_commit_sha: RustedGitKeyEvent,
	pub commit_amend: RustedGitKeyEvent,
	pub toggle_signoff: RustedGitKeyEvent,
	pub toggle_verify: RustedGitKeyEvent,
	pub copy: RustedGitKeyEvent,
	pub create_branch: RustedGitKeyEvent,
	pub rename_branch: RustedGitKeyEvent,
	pub select_branch: RustedGitKeyEvent,
	pub delete_branch: RustedGitKeyEvent,
	pub merge_branch: RustedGitKeyEvent,
	pub rebase_branch: RustedGitKeyEvent,
	pub reset_branch: RustedGitKeyEvent,
	pub compare_commits: RustedGitKeyEvent,
	pub tags: RustedGitKeyEvent,
	pub delete_tag: RustedGitKeyEvent,
	pub select_tag: RustedGitKeyEvent,
	pub push: RustedGitKeyEvent,
	pub open_file_tree: RustedGitKeyEvent,
	pub file_find: RustedGitKeyEvent,
	pub branch_find: RustedGitKeyEvent,
	pub force_push: RustedGitKeyEvent,
	pub fetch: RustedGitKeyEvent,
	pub pull: RustedGitKeyEvent,
	pub abort_merge: RustedGitKeyEvent,
	pub undo_commit: RustedGitKeyEvent,
	pub diff_hunk_next: RustedGitKeyEvent,
	pub diff_hunk_prev: RustedGitKeyEvent,
	pub stage_unstage_item: RustedGitKeyEvent,
	pub tag_annotate: RustedGitKeyEvent,
	pub view_submodules: RustedGitKeyEvent,
	pub view_remotes: RustedGitKeyEvent,
	pub update_remote_name: RustedGitKeyEvent,
	pub update_remote_url: RustedGitKeyEvent,
	pub add_remote: RustedGitKeyEvent,
	pub delete_remote: RustedGitKeyEvent,
	pub view_submodule_parent: RustedGitKeyEvent,
	pub update_submodule: RustedGitKeyEvent,
	pub commit_history_next: RustedGitKeyEvent,
	pub commit: RustedGitKeyEvent,
	pub newline: RustedGitKeyEvent,
	pub goto_line: RustedGitKeyEvent,
}

#[rustfmt::skip]
impl Default for KeysList {
	fn default() -> Self {
		Self {
			tab_status: RustedGitKeyEvent::new(KeyCode::Char('1'), KeyModifiers::empty()),
			tab_log: RustedGitKeyEvent::new(KeyCode::Char('2'),  KeyModifiers::empty()),
			tab_files: RustedGitKeyEvent::new(KeyCode::Char('3'),  KeyModifiers::empty()),
			tab_stashing: RustedGitKeyEvent::new(KeyCode::Char('4'),  KeyModifiers::empty()),
			tab_stashes: RustedGitKeyEvent::new(KeyCode::Char('5'),  KeyModifiers::empty()),
			tab_graph: RustedGitKeyEvent::new(KeyCode::Char('6'),  KeyModifiers::empty()),
			tab_create_pr: RustedGitKeyEvent::new(KeyCode::Char('7'),  KeyModifiers::empty()),
			tab_toggle: RustedGitKeyEvent::new(KeyCode::Tab,  KeyModifiers::empty()),
			tab_toggle_reverse: RustedGitKeyEvent::new(KeyCode::BackTab,  KeyModifiers::SHIFT),
			toggle_workarea: RustedGitKeyEvent::new(KeyCode::Char('w'),  KeyModifiers::empty()),
			exit: RustedGitKeyEvent::new(KeyCode::Char('c'),  KeyModifiers::CONTROL),
			quit: RustedGitKeyEvent::new(KeyCode::Char('q'),  KeyModifiers::empty()),
			exit_popup: RustedGitKeyEvent::new(KeyCode::Esc,  KeyModifiers::empty()),
			open_commit: RustedGitKeyEvent::new(KeyCode::Char('c'),  KeyModifiers::empty()),
			open_commit_editor: RustedGitKeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL),
			open_help: RustedGitKeyEvent::new(KeyCode::Char('h'),  KeyModifiers::empty()),
			open_options: RustedGitKeyEvent::new(KeyCode::Char('o'),  KeyModifiers::empty()),
			move_left: RustedGitKeyEvent::new(KeyCode::Left,  KeyModifiers::empty()),
			move_right: RustedGitKeyEvent::new(KeyCode::Right,  KeyModifiers::empty()),
			tree_collapse_recursive: RustedGitKeyEvent::new(KeyCode::Left,  KeyModifiers::SHIFT),
			tree_expand_recursive: RustedGitKeyEvent::new(KeyCode::Right,  KeyModifiers::SHIFT),
			home: RustedGitKeyEvent::new(KeyCode::Home,  KeyModifiers::empty()),
			end: RustedGitKeyEvent::new(KeyCode::End,  KeyModifiers::empty()),
			move_up: RustedGitKeyEvent::new(KeyCode::Up,  KeyModifiers::empty()),
			move_down: RustedGitKeyEvent::new(KeyCode::Down,  KeyModifiers::empty()),
			popup_up: RustedGitKeyEvent::new(KeyCode::Up,  KeyModifiers::empty()),
			popup_down: RustedGitKeyEvent::new(KeyCode::Down,  KeyModifiers::empty()),
			page_down: RustedGitKeyEvent::new(KeyCode::PageDown,  KeyModifiers::empty()),
			page_up: RustedGitKeyEvent::new(KeyCode::PageUp,  KeyModifiers::empty()),
			shift_up: RustedGitKeyEvent::new(KeyCode::Up,  KeyModifiers::SHIFT),
			shift_down: RustedGitKeyEvent::new(KeyCode::Down,  KeyModifiers::SHIFT),
			enter: RustedGitKeyEvent::new(KeyCode::Enter,  KeyModifiers::empty()),
			blame: RustedGitKeyEvent::new(KeyCode::Char('B'),  KeyModifiers::SHIFT),
			file_history: RustedGitKeyEvent::new(KeyCode::Char('H'),  KeyModifiers::SHIFT),
			edit_file: RustedGitKeyEvent::new(KeyCode::Char('e'),  KeyModifiers::empty()),
			status_stage_all: RustedGitKeyEvent::new(KeyCode::Char('a'),  KeyModifiers::empty()),
			status_reset_item: RustedGitKeyEvent::new(KeyCode::Char('D'),  KeyModifiers::SHIFT),
			diff_reset_lines: RustedGitKeyEvent::new(KeyCode::Char('d'),  KeyModifiers::empty()),
			status_ignore_file: RustedGitKeyEvent::new(KeyCode::Char('i'),  KeyModifiers::empty()),
			diff_stage_lines: RustedGitKeyEvent::new(KeyCode::Char('s'),  KeyModifiers::empty()),
			stashing_save: RustedGitKeyEvent::new(KeyCode::Char('s'),  KeyModifiers::empty()),
			stashing_toggle_untracked: RustedGitKeyEvent::new(KeyCode::Char('u'),  KeyModifiers::empty()),
			stashing_toggle_index: RustedGitKeyEvent::new(KeyCode::Char('i'),  KeyModifiers::empty()),
			stash_apply: RustedGitKeyEvent::new(KeyCode::Char('a'),  KeyModifiers::empty()),
			stash_open: RustedGitKeyEvent::new(KeyCode::Right,  KeyModifiers::empty()),
			stash_drop: RustedGitKeyEvent::new(KeyCode::Char('D'),  KeyModifiers::SHIFT),
			cmd_bar_toggle: RustedGitKeyEvent::new(KeyCode::Char('.'),  KeyModifiers::empty()),
			log_tag_commit: RustedGitKeyEvent::new(KeyCode::Char('t'),  KeyModifiers::empty()),
			log_mark_commit: RustedGitKeyEvent::new(KeyCode::Char(' '),  KeyModifiers::empty()),
			log_checkout_commit: RustedGitKeyEvent { code: KeyCode::Char('S'), modifiers: KeyModifiers::SHIFT },
			log_reset_commit: RustedGitKeyEvent { code: KeyCode::Char('R'), modifiers: KeyModifiers::SHIFT },
			log_reword_commit: RustedGitKeyEvent { code: KeyCode::Char('r'), modifiers: KeyModifiers::empty() },
			log_find: RustedGitKeyEvent { code: KeyCode::Char('f'), modifiers: KeyModifiers::empty() },
			find_commit_sha: RustedGitKeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL),
			commit_amend: RustedGitKeyEvent::new(KeyCode::Char('a'),  KeyModifiers::CONTROL),
			toggle_signoff: RustedGitKeyEvent::new(KeyCode::Char('s'),  KeyModifiers::CONTROL),
			toggle_verify: RustedGitKeyEvent::new(KeyCode::Char('f'),  KeyModifiers::CONTROL),
			copy: RustedGitKeyEvent::new(KeyCode::Char('y'),  KeyModifiers::empty()),
			create_branch: RustedGitKeyEvent::new(KeyCode::Char('c'),  KeyModifiers::empty()),
			rename_branch: RustedGitKeyEvent::new(KeyCode::Char('r'),  KeyModifiers::empty()),
			select_branch: RustedGitKeyEvent::new(KeyCode::Char('b'),  KeyModifiers::empty()),
			delete_branch: RustedGitKeyEvent::new(KeyCode::Char('D'),  KeyModifiers::SHIFT),
			merge_branch: RustedGitKeyEvent::new(KeyCode::Char('m'),  KeyModifiers::empty()),
			rebase_branch: RustedGitKeyEvent::new(KeyCode::Char('R'),  KeyModifiers::SHIFT),
			reset_branch: RustedGitKeyEvent::new(KeyCode::Char('s'),  KeyModifiers::empty()),
			compare_commits: RustedGitKeyEvent::new(KeyCode::Char('C'),  KeyModifiers::SHIFT),
			tags: RustedGitKeyEvent::new(KeyCode::Char('T'),  KeyModifiers::SHIFT),
			delete_tag: RustedGitKeyEvent::new(KeyCode::Char('D'),  KeyModifiers::SHIFT),
			select_tag: RustedGitKeyEvent::new(KeyCode::Enter,  KeyModifiers::empty()),
			push: RustedGitKeyEvent::new(KeyCode::Char('p'),  KeyModifiers::empty()),
			force_push: RustedGitKeyEvent::new(KeyCode::Char('P'),  KeyModifiers::SHIFT),
			undo_commit: RustedGitKeyEvent::new(KeyCode::Char('U'),  KeyModifiers::SHIFT),
			fetch: RustedGitKeyEvent::new(KeyCode::Char('F'),  KeyModifiers::SHIFT),
			pull: RustedGitKeyEvent::new(KeyCode::Char('f'),  KeyModifiers::empty()),
			abort_merge: RustedGitKeyEvent::new(KeyCode::Char('A'),  KeyModifiers::SHIFT),
			open_file_tree: RustedGitKeyEvent::new(KeyCode::Char('F'),  KeyModifiers::SHIFT),
			file_find: RustedGitKeyEvent::new(KeyCode::Char('f'),  KeyModifiers::empty()),
			branch_find: RustedGitKeyEvent::new(KeyCode::Char('f'),  KeyModifiers::empty()),
			diff_hunk_next: RustedGitKeyEvent::new(KeyCode::Char('n'),  KeyModifiers::empty()),
			diff_hunk_prev: RustedGitKeyEvent::new(KeyCode::Char('p'),  KeyModifiers::empty()),
			stage_unstage_item: RustedGitKeyEvent::new(KeyCode::Enter,  KeyModifiers::empty()),
			tag_annotate: RustedGitKeyEvent::new(KeyCode::Char('a'),  KeyModifiers::CONTROL),
			view_submodules: RustedGitKeyEvent::new(KeyCode::Char('S'),  KeyModifiers::SHIFT),
			view_remotes: RustedGitKeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
			update_remote_name: RustedGitKeyEvent::new(KeyCode::Char('n'),KeyModifiers::NONE),
			update_remote_url: RustedGitKeyEvent::new(KeyCode::Char('u'),KeyModifiers::NONE),
			add_remote: RustedGitKeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
			delete_remote: RustedGitKeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
			view_submodule_parent: RustedGitKeyEvent::new(KeyCode::Char('p'),  KeyModifiers::empty()),
			update_submodule: RustedGitKeyEvent::new(KeyCode::Char('u'),  KeyModifiers::empty()),
			commit_history_next: RustedGitKeyEvent::new(KeyCode::Char('n'),  KeyModifiers::CONTROL),
			commit: RustedGitKeyEvent::new(KeyCode::Char('d'),  KeyModifiers::CONTROL),
			newline: RustedGitKeyEvent::new(KeyCode::Enter,  KeyModifiers::empty()),
			goto_line: RustedGitKeyEvent::new(KeyCode::Char('L'),  KeyModifiers::SHIFT),
		}
	}
}

impl KeysList {
	pub fn init(file: PathBuf) -> Self {
		let mut keys_list = Self::default();
		if let Ok(f) = File::open(file) {
			match ron::de::from_reader(f) {
				Ok(patch) => keys_list.apply(patch),
				Err(e) => {
					log::error!("KeysList parse error: {e}");
				}
			}
		}
		keys_list
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::io::Write;
	use tempfile::NamedTempFile;

	#[test]
	fn test_apply_vim_style_example() {
		let mut keys_list = KeysList::default();
		let f = File::open("vim_style_key_config.ron")
			.expect("vim style config should exist");
		let patch = ron::de::from_reader(f)
			.expect("vim style config format incorrect");
		keys_list.apply(patch);
	}

	#[test]
	fn test_smoke() {
		let mut file = NamedTempFile::new().unwrap();

		writeln!(
			file,
			r#"
(
	move_down: Some(( code: Char('j'), modifiers: "CONTROL")),
	move_up: Some((code: Char('h'), modifiers: ""))
)
"#
		)
		.unwrap();

		let keys = KeysList::init(file.path().to_path_buf());

		assert_eq!(keys.move_right, KeysList::default().move_right);
		assert_eq!(
			keys.move_down,
			RustedGitKeyEvent::new(
				KeyCode::Char('j'),
				KeyModifiers::CONTROL
			)
		);
		assert_eq!(
			keys.move_up,
			RustedGitKeyEvent::new(
				KeyCode::Char('h'),
				KeyModifiers::NONE
			)
		);
	}
}
