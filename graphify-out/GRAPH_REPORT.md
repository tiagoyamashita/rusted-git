# Graph Report - workspace  (2026-07-21)

## Corpus Check
- 192 files · ~1,437,262 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4235 nodes · 10214 edges · 199 communities (183 shown, 16 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 707 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `dbbafaff`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SharedKeyConfig
- strings.rs
- .update
- BlameFilePopup
- mod.rs
- .new
- PushPopup
- Status
- RefGraph
- DiffComponent
- Revlog
- lib.rs
- SubmodulesListPopup
- RemoteListPopup
- TagListPopup
- CreatePrTab
- PushTagsPopup
- mod.rs
- StatusTreeComponent
- FileRevlogPopup
- StashList
- TextInputComponent
- BranchListPopup
- App
- repo_init_empty
- Theme
- FileTreeItem
- CommitPopup
- ResetPopup
- logwalker.rs
- stash.rs
- tree.rs
- LogSearchPopupPopup
- CommitId
- DetailsComponent
- RepoPath
- AsyncGitNotification
- repo
- SyntaxText
- CommitList
- ChangesComponent
- RevisionFilesComponent
- hooks.rs
- PullPopup
- Changelog
- EventState
- stage_add_file
- SyntaxTextComponent
- Options
- CheckoutOptionPopup
- debug_cmd_print
- utils.rs
- HookPaths
- CompareCommitsPopup
- OptionsPopup
- BasicAuthCredential
- Callbacks
- Input
- reflow.rs
- AsyncDiff
- CompareDetailsComponent
- main.rs
- CommitDetailsComponent
- RevisionFilesPopup
- FilesTab
- AsyncStatus
- CreateRemotePopup
- VerticalScroll
- AsyncCreatePrJob
- FetchPopup
- rebase.rs
- RunParams
- CommandInfo
- key_match
- RenameRemotePopup
- Stashing
- Error
- diff.rs
- KeyConfig
- FuzzyFindPopup
- AsyncCommitFiles
- get_commit_details
- README.md
- CreateBranchPopup
- HelpPopup
- UpdateRemoteUrlPopup
- Various Package Managers
- AsyncSingleJob
- get_config_string
- CommandBar
- mod.rs
- AsyncSingleJob<J>
- get_status
- MsgPopup
- StashMsgPopup
- draw_list
- CommitDetails
- Gitui
- Queue
- LogEntry
- Themes
- DrawableComponent
- mod.rs
- AsyncRemoteTagsJob
- get_commit_files
- .get_entry_to_add
- draw_scrollbar
- AsyncBranchesJob
- DiffLinePosition
- DiffOptions
- add_to_ignore
- .init
- HorizontalScroll
- apply_selection
- FAQ.md
- [0.25.0] - 2024-02-21
- Spinner
- AsyncFetchJob
- ShowUntrackedFilesConfig
- .forwarder
- 6. <a name="installation"></a> Installation <small><sup>[Top ▲](#table-of-contents)</sup></small>
- reword
- lib.rs
- [0.23.0] - 2023-06-19
- [0.24.0] - 2023-08-27
- get_file_diff_patch
- commitlist.rs
- [0.26.3] - 2024-06-02
- ScrollType
- Contributing
- 7. <a name="build"></a> Build <small><sup>[Top ▲](#table-of-contents)</sup></small>
- .draw
- T
- blame_file
- write_commit_file
- [0.11.0] - 2021-12-20
- ScopeTimeLog
- style_detail
- [0.12.0] - 2021-03-03
- build.rs
- Error
- [0.13.0] - 2021-03-15 - Happy Birthday GitUI 🥳
- .draw
- .draw
- invalid_utf8
- .new
- .new
- [0.14.0] - 2021-04-11
- [0.15.0] - 2021-04-27
- [0.16.0] - 2021-05-28
- [0.16.1] - 2021-06-06
- [0.17.1] - 2021-09-10
- [0.19] - 2021-12-08 - Bare Repo Support
- [0.20] - 2022-01-25 - Tag Annotations
- [0.21.0] - 2022-08-17
- [0.26.0+1] - 2024-04-14
- [0.27.0] - 2025-01-14
- [0.28.0] - 2025-12-14
- [0.2.2] - 2020-05-10
- [0.2.5] - 2020-05-16
- [0.3.0] - 2020-05-20
- [0.4.0] - 2020-05-25
- [0.5.0] - 2020-06-01
- [0.6.0] - 2020-06-09
- [0.7.0] - 2020-06-15
- [0.8.0] - 2020-07-06
- [0.9.1] - 2020-07-30
- Unreleased
- .new
- [0.10.0] - 2020-08-29
- [0.17.0] - 2021-08-21
- [0.18] - 2021-10-11
- [0.20.1] - 2022-01-26
- [0.22.0] - 2022-11-19
- [0.22.1] - 2022-11-22
- [0.25.2] - 2024-03-22
- [0.26.2] - 2024-04-17
- [0.28.1] - 2026-03-21
- git2-hooks
- README.md
- README.md
- README.md
- README.md
- README.md
- clipboard.rs
- FileRevOpen
- ProgressPercent
- .event
- stateful_paragraph.rs
- ShowUntrackedFilesConfig
- asyncgit/src/lib.rs
- .draw_revlog
- [0.16.2] - 2021-07-10

## God Nodes (most connected - your core abstractions)
1. `RepoPath` - 187 edges
2. `CommitId` - 141 edges
3. `CommandText` - 126 edges
4. `repo()` - 120 edges
5. `App` - 82 edges
6. `CommitList` - 71 edges
7. `CommandInfo` - 64 edges
8. `Environment` - 61 edges
9. `Status` - 59 edges
10. `EventState` - 58 edges

## Surprising Connections (you probably didn't know these)
- `rusted_git_starts()` --calls--> `repo_init_suffix()`  [INFERRED]
  src/rusted_git.rs → git2-testing/src/lib.rs
- `AsyncSyntaxJob` --implements--> `AsyncJob`  [EXTRACTED]
  src/ui/syntax_text.rs → asyncgit/src/asyncjob/mod.rs
- `RevisionFilesComponent` --references--> `AsyncSingleJob`  [EXTRACTED]
  src/components/revision_files.rs → asyncgit/src/asyncjob/mod.rs
- `SyntaxTextComponent` --references--> `AsyncSingleJob`  [EXTRACTED]
  src/components/syntax_text.rs → asyncgit/src/asyncjob/mod.rs
- `BlameProcess` --references--> `AsyncSingleJob`  [EXTRACTED]
  src/popups/blame_file.rs → asyncgit/src/asyncjob/mod.rs

## Import Cycles
- None detected.

## Communities (199 total, 16 thin omitted)

### Community 0 - "SharedKeyConfig"
Cohesion: 0.03
Nodes (121): CommandText, abort_merge(), abort_rebase(), abort_revert(), blame_file(), branch_popup_rebase(), close_fuzzy_finder(), close_msg() (+113 more)

### Community 1 - "strings.rs"
Cohesion: 0.04
Nodes (101): blame_title(), cmd_splitter(), commit_editor_msg(), commit_first_line_warning(), commit_msg(), commit_reword_title(), commit_title(), commit_title_amend() (+93 more)

### Community 2 - ".update"
Cohesion: 0.05
Nodes (61): String, StatusItem, Index, IndexMut, Output, FileTreeItem, FileTreeItemKind, FileTreeItems (+53 more)

### Community 3 - "BlameFilePopup"
Cohesion: 0.09
Nodes (27): find_truncate_point(), String, string_width_align(), time_to_string(), AsyncNotification, BlameFileOpen, BlameFilePopup, BlameProcess (+19 more)

### Community 4 - "mod.rs"
Cohesion: 0.11
Nodes (37): AsRef, commit_merge_with_head(), merge_upstream_commit(), Commit, Option, Repository, Result, test_merge_normal() (+29 more)

### Community 5 - ".new"
Cohesion: 0.07
Nodes (47): FileTreeItems, get_visible(), BTreeSet, FileTreeItem, HashMap, Option, Path, PathBuf (+39 more)

### Community 6 - "PushPopup"
Cohesion: 0.06
Nodes (35): AsyncPush, PushRequest, PushState, Arc, Mutex, Option, Result, Self (+27 more)

### Community 7 - "Status"
Cohesion: 0.08
Nodes (22): repo_state(), RepoState, From, Result, Self, RepositoryState, DiffTarget, Focus (+14 more)

### Community 8 - "RefGraph"
Cohesion: 0.07
Nodes (46): assign_lanes(), cid(), draw_cells(), draw_fork_connector(), draw_merge_connector(), ensure_width(), fill_horizontal(), GraphCell (+38 more)

### Community 9 - "DiffComponent"
Cohesion: 0.06
Nodes (24): Direction, Current, diff_component_opens_editor_for_current_file(), DiffComponent, Cell, Event, Frame, Line (+16 more)

### Community 10 - "Revlog"
Cohesion: 0.07
Nodes (30): diff_contains_file(), filter_commit_by_search(), LogFilterSearch, LogFilterSearchOptions, Default, Diff, Self, SharedCommitFilterFn (+22 more)

### Community 11 - "lib.rs"
Cohesion: 0.10
Nodes (55): HooksError, Error, branch_update(), create_hook(), create_hook_in_path(), head_branch(), hook_available(), HookResult (+47 more)

### Community 12 - "SubmodulesListPopup"
Cohesion: 0.09
Nodes (29): get_submodules(), Option, PathBuf, Repository, Result, String, Vec, submodule_parent_info() (+21 more)

### Community 13 - "RemoteListPopup"
Cohesion: 0.14
Nodes (13): RemoteListPopup, Cell, Event, Frame, Rect, RepoPathRef, Result, Self (+5 more)

### Community 14 - "TagListPopup"
Cohesion: 0.06
Nodes (38): AsyncRemoteTagsJob, JobState, Arc, Mutex, Notification, Option, Progress, Result (+30 more)

### Community 15 - "CreatePrTab"
Cohesion: 0.07
Nodes (24): CreatePrTab, default_base_branch(), Focus, Event, Frame, Option, Rect, RepoPathRef (+16 more)

### Community 16 - "PushTagsPopup"
Cohesion: 0.09
Nodes (23): AsyncPushTags, PushState, PushTagsRequest, Arc, Mutex, Option, Result, Self (+15 more)

### Community 17 - "mod.rs"
Cohesion: 0.10
Nodes (40): branch_compare_upstream(), branch_set_upstream_after_push(), BranchCompare, BranchDetails, BranchInfo, checkout_commit(), checkout_remote_branch(), clone_branch_commit_push() (+32 more)

### Community 18 - "StatusTreeComponent"
Cohesion: 0.09
Nodes (20): FileTreeItemKind, Cell, Event, FileTreeItem, Frame, MoveSelection, Option, Rect (+12 more)

### Community 19 - "FileRevlogPopup"
Cohesion: 0.20
Nodes (8): FileRevlogPopup, Cell, RepoPathRef, Result, SharedKeyConfig, SharedOptions, SharedTheme, TableState

### Community 20 - "StashList"
Cohesion: 0.13
Nodes (29): AsyncProgress, ProgressNotification, push_branch(), push_raw(), PushType, Clone, Option, PackBuilderStage (+21 more)

### Community 21 - "TextInputComponent"
Cohesion: 0.06
Nodes (35): OnceCell, InputType, Cell, Event, Frame, Option, Rect, Result (+27 more)

### Community 22 - "BranchListPopup"
Cohesion: 0.12
Nodes (14): BranchListPopup, BranchType, Cell, Event, Frame, Option, Rect, RepoPathRef (+6 more)

### Community 23 - "App"
Cohesion: 0.10
Nodes (19): App, Environment, Cell, Event, Frame, NeedsUpdate, Option, Rect (+11 more)

### Community 24 - "repo_init_empty"
Cohesion: 0.13
Nodes (33): test_empty_repo(), repo_init_empty(), create_signed_commit(), GPGSign, gpgsm_import_p12(), Box, Commit, Option (+25 more)

### Community 25 - "Theme"
Cohesion: 0.13
Nodes (10): Color, Default, PathBuf, Result, Self, String, Style, test_smoke() (+2 more)

### Community 26 - "FileTreeItem"
Cohesion: 0.10
Nodes (15): FileTreeItem, FileTreeItemKind, PathCollapsed, Eq, Option, Ord, Ordering, PartialEq (+7 more)

### Community 27 - "CommitPopup"
Cohesion: 0.07
Nodes (24): CommitPopup, CommitResult, Mode, Event, Frame, Option, Rect, RepoPathRef (+16 more)

### Community 28 - "ResetPopup"
Cohesion: 0.09
Nodes (19): BranchName, Option, RepoPathRef, Result, Self, String, ResetPopup, Event (+11 more)

### Community 29 - "logwalker.rs"
Cohesion: 0.13
Nodes (16): LogWalker, LogWalker<'a>, BinaryHeap, Commit, Eq, HashSet, Option, Ord (+8 more)

### Community 30 - "stash.rs"
Cohesion: 0.21
Nodes (20): get_stash_index(), get_stashes(), Option, Repository, Result, Vec, stash_apply(), stash_drop() (+12 more)

### Community 31 - "tree.rs"
Cohesion: 0.05
Nodes (48): path_cmp(), Ordering, Path, PathBuf, Repository, Result, String, Tree (+40 more)

### Community 32 - "LogSearchPopupPopup"
Cohesion: 0.14
Nodes (14): LogSearchPopupPopup, PopupMode, Event, Frame, Line, Option, Rect, RepoPathRef (+6 more)

### Community 33 - "CommitId"
Cohesion: 0.13
Nodes (24): CommitId, CommitInfo, get_commit_info(), get_commits_info(), get_message(), gix_get_message(), gix::ObjectId, Oid (+16 more)

### Community 34 - "DetailsComponent"
Cohesion: 0.10
Nodes (18): DetailsComponent, get_wrapped_lines(), Cell, CommitTags, Cow, Event, Frame, Line (+10 more)

### Community 35 - "RepoPath"
Cohesion: 0.09
Nodes (37): commit_revert(), revert_commit(), revert_head(), Result, abort_pending_rebase(), abort_pending_state(), continue_pending_rebase(), merge_branch() (+29 more)

### Community 36 - "AsyncGitNotification"
Cohesion: 0.19
Nodes (16): AsyncGitNotification, AsyncLog, AsyncLogResult, FetchStatus, Arc, AtomicBool, Duration, Mutex (+8 more)

### Community 37 - "repo"
Cohesion: 0.17
Nodes (34): need_username_password_for_fetch(), need_username_password_for_push(), add_remote(), delete_remote(), fetch(), fetch_all(), fetch_from_remote(), get_current_branch() (+26 more)

### Community 38 - "SyntaxText"
Cohesion: 0.11
Nodes (23): Range, AsyncProgressBuffer, AsyncSyntaxJob, JobState, ratatui::text::Text<'a>, Arc, Duration, From (+15 more)

### Community 39 - "CommitList"
Cohesion: 0.11
Nodes (12): BTreeMap, CommitList, Box, Cell, Default, IndexSet, Instant, Rc (+4 more)

### Community 40 - "ChangesComponent"
Cohesion: 0.11
Nodes (12): ChangesComponent, Event, FileTreeItem, Frame, Option, Rect, RepoPathRef, Result (+4 more)

### Community 41 - "RevisionFilesComponent"
Cohesion: 0.09
Nodes (23): Direction, FileTree, MoveSelection, BTreeSet, Cell, FileTreeItems, Item, Iterator (+15 more)

### Community 42 - "hooks.rs"
Cohesion: 0.17
Nodes (28): advertised_remote_refs(), create_hook_in_path(), get_remote_ref_for_push(), HookResult, hooks_commit_msg(), hooks_post_commit(), hooks_pre_commit(), hooks_pre_push() (+20 more)

### Community 43 - "PullPopup"
Cohesion: 0.27
Nodes (10): AsyncPull, FetchRequest, FetchState, Arc, Mutex, Option, Result, Self (+2 more)

### Community 44 - "Changelog"
Cohesion: 0.09
Nodes (22): [0.10.0] - 2020-08-29, [0.10.1] - 2020-09-01, [0.24.1] - 2023-08-30, [0.24.2] - 2023-09-03, [0.24.3] - 2023-09-09, [0.25.1] - 2024-02-23, [0.2.3] - 2020-05-12, [0.2.6] - 2020-05-18 (+14 more)

### Community 45 - "EventState"
Cohesion: 0.07
Nodes (23): InspectCommitOpen, InspectCommitPopup, CommitTags, Event, Frame, Option, Rect, Result (+15 more)

### Community 46 - "stage_add_file"
Cohesion: 0.21
Nodes (22): amend(), commit(), commit_message_prettify(), count_commits(), Error, Option, Repository, Result (+14 more)

### Community 47 - "SyntaxTextComponent"
Cohesion: 0.12
Nodes (15): Either, Cell, Event, Frame, MoveSelection, Option, Rect, RepoPathRef (+7 more)

### Community 48 - "Options"
Cohesion: 0.13
Nodes (10): Options, OptionsData, Option, PathBuf, RepoPathRef, Result, Self, SharedOptions (+2 more)

### Community 49 - "CheckoutOptionPopup"
Cohesion: 0.12
Nodes (13): CheckoutOptionPopup, Event, Frame, Line, Option, Rect, Result, Self (+5 more)

### Community 50 - "debug_cmd_print"
Cohesion: 0.27
Nodes (14): debug_cmd(), debug_cmd_print(), String, reset_repo(), reset_stage(), reset_workdir(), ResetType, Result (+6 more)

### Community 51 - "utils.rs"
Cohesion: 0.15
Nodes (27): bytes2string(), get_head(), get_head_refname(), get_head_repo(), get_head_tuple(), Head, read_file(), repo_dir() (+19 more)

### Community 52 - "HookPaths"
Cohesion: 0.16
Nodes (15): Command, CommandExt, HookPaths, is_executable(), Command, Option, Path, PathBuf (+7 more)

### Community 53 - "CompareCommitsPopup"
Cohesion: 0.15
Nodes (11): CompareCommitsPopup, Event, Frame, Option, Rect, RepoPathRef, Result, Self (+3 more)

### Community 54 - "OptionsPopup"
Cohesion: 0.15
Nodes (12): AppOption, OptionsPopup, Event, Frame, Line, Rect, Result, Self (+4 more)

### Community 55 - "BasicAuthCredential"
Cohesion: 0.14
Nodes (12): BasicAuthCredential, extract_cred_from_url(), extract_username_password(), extract_username_password_for_fetch(), extract_username_password_for_push(), need_username_password(), Option, Result (+4 more)

### Community 56 - "Callbacks"
Cohesion: 0.13
Nodes (16): Callbacks, CallbackStats, Arc, AtomicBool, Mutex, Option, PackBuilderStage, Progress (+8 more)

### Community 57 - "Input"
Cohesion: 0.13
Nodes (16): Condvar, Input, InputEvent, InputState, Arc, AtomicBool, Duration, Event (+8 more)

### Community 58 - "reflow.rs"
Cohesion: 0.10
Nodes (33): Composer, line_composer_char_plus_lots_of_spaces(), line_composer_double_width_chars(), line_composer_leading_whitespace_removal(), line_composer_long_sentence(), line_composer_long_word(), line_composer_lots_of_spaces(), line_composer_max_line_width_of_1() (+25 more)

### Community 59 - "AsyncDiff"
Cohesion: 0.18
Nodes (16): AsyncDiff, DiffParams, DiffType, LastResult, Request, A, Arc, AtomicUsize (+8 more)

### Community 60 - "CompareDetailsComponent"
Cohesion: 0.11
Nodes (14): CompareDetailsComponent, Event, Frame, Line, Option, Rect, RepoPathRef, Result (+6 more)

### Community 61 - "main.rs"
Cohesion: 0.17
Nodes (21): QuitState, AsyncAppNotification, draw(), ensure_valid_path(), main(), QueueEvent, B, Error (+13 more)

### Community 62 - "CommitDetailsComponent"
Cohesion: 0.15
Nodes (10): CommitDetailsComponent, CommitTags, Event, Frame, Option, Rect, Result, Self (+2 more)

### Community 63 - "RevisionFilesPopup"
Cohesion: 0.15
Nodes (18): tree_nav(), key_match(), KeyEvent, KeysList, Default, From, KeyCode, KeyModifiers (+10 more)

### Community 64 - "FilesTab"
Cohesion: 0.19
Nodes (15): AsyncBlame, BlameParams, LastResult, Request, A, Arc, AtomicUsize, Mutex (+7 more)

### Community 65 - "AsyncStatus"
Cohesion: 0.18
Nodes (15): AsyncStatus, Request, A, Arc, AtomicUsize, Mutex, Option, R (+7 more)

### Community 66 - "CreateRemotePopup"
Cohesion: 0.14
Nodes (13): validate_remote_name(), CreateRemotePopup, Event, Frame, Rect, RepoPathRef, Result, Self (+5 more)

### Community 67 - "VerticalScroll"
Cohesion: 0.19
Nodes (7): calc_scroll_top(), Cell, Self, test_scroll_bottom_into_view(), test_scroll_top_into_view(), test_scroll_with_pageup_pagedown(), VerticalScroll

### Community 68 - "AsyncCreatePrJob"
Cohesion: 0.23
Nodes (12): AsyncCreatePrJob, create_pull_request(), CreatePrRequest, JobState, Arc, Mutex, Notification, Option (+4 more)

### Community 69 - "FetchPopup"
Cohesion: 0.05
Nodes (31): CredComponent, Event, Frame, Rect, Result, Self, SharedKeyConfig, Vec (+23 more)

### Community 70 - "rebase.rs"
Cohesion: 0.22
Nodes (21): AnnotatedCommit, checkout_branch(), abort_rebase(), conflict_free_rebase(), continue_rebase(), get_rebase_progress(), parent_ids(), rebase() (+13 more)

### Community 71 - "RunParams"
Cohesion: 0.17
Nodes (16): RunParams, AsyncCommitFilterJob, CommitFilterResult, JobState, Arc, AtomicBool, Duration, Instant (+8 more)

### Community 72 - "CommandInfo"
Cohesion: 0.09
Nodes (20): CommandInfo, Self, Vec, command_pump(), CommandBlocking, Component, Direction, event_pump() (+12 more)

### Community 73 - "key_match"
Cohesion: 0.15
Nodes (12): RenameBranchPopup, Event, Frame, Option, Rect, RepoPathRef, Result, Self (+4 more)

### Community 74 - "RenameRemotePopup"
Cohesion: 0.15
Nodes (12): RenameRemotePopup, Event, Frame, Option, Rect, RepoPathRef, Result, Self (+4 more)

### Community 75 - "Stashing"
Cohesion: 0.15
Nodes (11): Event, Frame, Line, Rect, RepoPathRef, Result, Self, SharedKeyConfig (+3 more)

### Community 76 - "Error"
Cohesion: 0.18
Nodes (14): Error, GixError, Box, Error, From, Self, String, T (+6 more)

### Community 77 - "diff.rs"
Cohesion: 0.19
Nodes (22): DiffOptions, FileDiff, get_diff(), get_diff_commit(), get_diff_commits(), get_diff_raw(), Hunk, new_file_content() (+14 more)

### Community 78 - "KeyConfig"
Cohesion: 0.15
Nodes (14): Q, create_symlink(), KeyConfig, Error, KeyCode, KeyModifiers, Option, P (+6 more)

### Community 79 - "FuzzyFindPopup"
Cohesion: 0.16
Nodes (12): FuzzyFinderTarget, FuzzyFindPopup, Event, Frame, Option, Rect, Result, Self (+4 more)

### Community 80 - "AsyncCommitFiles"
Cohesion: 0.19
Nodes (14): AsyncCommitFiles, CommitFilesParams, Request, A, Arc, AtomicUsize, From, Mutex (+6 more)

### Community 81 - "get_commit_details"
Cohesion: 0.29
Nodes (11): get_author_of_commit(), get_commit_details(), get_committer_of_commit(), Commit, Result, Self, Signature, test_commit_message_combine() (+3 more)

### Community 82 - "README.md"
Cohesion: 0.13
Nodes (14): 10. <a name="theme"></a> Color Theme <small><sup>[Top ▲](#table-of-contents)</sup></small>, 11. <a name="bindings"></a> Key Bindings <small><sup>[Top ▲](#table-of-contents)</sup></small>, 12. <a name="sponsoring"></a> Sponsoring <small><sup>[Top ▲](#table-of-contents)</sup></small>, 13. <a name="inspiration"></a> Inspiration <small><sup>[Top ▲](#table-of-contents)</sup></small>, 14. <a name="contributing"></a> Contributing <small><sup>[Top ▲](#table-of-contents)</sup></small>, 15. <a name="contributors"></a> Contributors <small><sup>[Top ▲](#table-of-contents)</sup></small>, 1. <a name="features"></a> Features <small><sup>[Top ▲](#table-of-contents)</sup></small>, 2. <a name="motivation"></a> Motivation <small><sup>[Top ▲](#table-of-contents)</sup></small> (+6 more)

### Community 83 - "CreateBranchPopup"
Cohesion: 0.17
Nodes (10): CreateBranchPopup, Event, Frame, Rect, RepoPathRef, Result, Self, SharedKeyConfig (+2 more)

### Community 84 - "HelpPopup"
Cohesion: 0.17
Nodes (10): HelpPopup, Event, Frame, Line, Rect, Result, Self, SharedKeyConfig (+2 more)

### Community 85 - "UpdateRemoteUrlPopup"
Cohesion: 0.16
Nodes (11): Event, Frame, Option, Rect, RepoPathRef, Result, Self, SharedKeyConfig (+3 more)

### Community 86 - "Various Package Managers"
Cohesion: 0.14
Nodes (14): [Anaconda](https://anaconda.org/conda-forge/rusted-git), [Arch Linux](https://archlinux.org/packages/extra/x86_64/rusted-git/), [Chocolatey](https://chocolatey.org/packages/rusted-git) (Windows), Fedora, Gentoo, Homebrew (macOS), [MacPorts (macOS)](https://ports.macports.org/port/rusted-git/details/), [Mise](https://github.com/jdx/mise) (+6 more)

### Community 87 - "AsyncSingleJob"
Cohesion: 0.15
Nodes (18): AsyncJob, AsyncSingleJob, Arc, AtomicBool, Clone, Mutex, Notification, Progress (+10 more)

### Community 88 - "get_config_string"
Cohesion: 0.28
Nodes (14): get_config_string(), get_config_string_repo(), push_default_strategy_config_repo(), PushDefaultStrategyConfig, Error, Option, Repository, Result (+6 more)

### Community 89 - "CommandBar"
Cohesion: 0.15
Nodes (11): Command, CommandBar, DrawListEntry, Command, Frame, Rect, Self, SharedKeyConfig (+3 more)

### Community 90 - "mod.rs"
Cohesion: 0.13
Nodes (13): ConfirmPopup, Event, Frame, Option, Rect, Result, Self, SharedKeyConfig (+5 more)

### Community 91 - "AsyncSingleJob<J>"
Cohesion: 0.20
Nodes (7): AsyncSingleJob<J>, Option, P, Result, T, RunParams<T, P>, J

### Community 92 - "get_status"
Cohesion: 0.17
Nodes (15): AsyncTags, AsyncTagsJob, JobState, Arc, Duration, Instant, Mutex, Notification (+7 more)

### Community 93 - "MsgPopup"
Cohesion: 0.19
Nodes (9): MsgPopup, Event, Frame, Rect, Result, Self, SharedKeyConfig, SharedTheme (+1 more)

### Community 94 - "StashMsgPopup"
Cohesion: 0.16
Nodes (10): Event, Frame, Rect, RepoPathRef, Result, Self, SharedKeyConfig, Vec (+2 more)

### Community 95 - "draw_list"
Cohesion: 0.21
Nodes (14): L, draw_list(), draw_list_block(), Block, Buffer, Frame, Option, Rect (+6 more)

### Community 96 - "CommitDetails"
Cohesion: 0.33
Nodes (6): CommitDetails, CommitMessage, CommitSignature, Option, String, WrappedCommitMessage

### Community 97 - "Gitui"
Cohesion: 0.19
Nodes (16): test_diff_delta_size(), test_diff_delta_size_commit(), LogWalkerWithoutFilter, LogWalkerWithoutFilter<'a>, Result, test_limit(), test_logwalker(), test_logwalker_with_filter() (+8 more)

### Community 98 - "Queue"
Cohesion: 0.11
Nodes (18): PopupStack, Option, Vec, Action, AppTabs, InternalEvent, Queue, ResetItem (+10 more)

### Community 99 - "LogEntry"
Cohesion: 0.13
Nodes (12): BoxStr, Iter, emojifi_string(), String, LogEntry, DateTime, From, IndexSet (+4 more)

### Community 100 - "Themes"
Cohesion: 0.33
Nodes (6): Configuration, Customizing line breaks, Customizing selection, Preset Themes, Syntax Highlighting, Themes

### Community 101 - "DrawableComponent"
Cohesion: 0.18
Nodes (10): GotoLinePopup, Event, Frame, Rect, Result, Self, SharedKeyConfig, SharedTheme (+2 more)

### Community 102 - "mod.rs"
Cohesion: 0.29
Nodes (9): centered_rect(), centered_rect_absolute(), rect_inside(), From, Rect, Self, Size, test_small_rect_in_rect() (+1 more)

### Community 103 - "AsyncRemoteTagsJob"
Cohesion: 0.21
Nodes (11): B, Error, Instant, KeyCode, KeyModifiers, Receiver, Result, Self (+3 more)

### Community 104 - "get_commit_files"
Cohesion: 0.27
Nodes (16): get_commit_diff(), get_commit_files(), get_compare_commits_diff(), OldNew, Diff, HashSet, Option, Repository (+8 more)

### Community 105 - ".get_entry_to_add"
Cohesion: 0.21
Nodes (6): DateTime, Line, Local, Option, String, Tags

### Community 106 - "draw_scrollbar"
Cohesion: 0.08
Nodes (21): HorizontalScrollType, calc_scroll_right(), HorizontalScroll, Cell, Frame, Rect, Self, SharedTheme (+13 more)

### Community 107 - "AsyncBranchesJob"
Cohesion: 0.20
Nodes (10): AsyncBranchesJob, JobState, Arc, Mutex, Notification, Option, Progress, Result (+2 more)

### Community 108 - "DiffLinePosition"
Cohesion: 0.23
Nodes (9): DiffLine, DiffLinePosition, DiffLineType, HunkHeader, Box, From, PartialEq, Self (+1 more)

### Community 109 - "DiffOptions"
Cohesion: 0.44
Nodes (8): find_hunk_index(), reset_hunk(), reset_untracked_file_which_will_not_find_hunk(), Diff, Option, Result, stage_hunk(), unstage_hunk()

### Community 110 - "add_to_ignore"
Cohesion: 0.29
Nodes (13): add_to_ignore(), file_ends_with_newline(), read_lines(), P, Path, Result, test_append(), test_append_no_newline_at_end() (+5 more)

### Community 112 - ".init"
Cohesion: 0.14
Nodes (14): Alignment, get_line_offset(), Block, Buffer, Option, Rect, Self, Style (+6 more)

### Community 113 - "HorizontalScroll"
Cohesion: 0.14
Nodes (10): OsStr, DrawableComponent, ExternalEditorPopup, Event, Frame, Rect, Result, Self (+2 more)

### Community 114 - "apply_selection"
Cohesion: 0.31
Nodes (7): add_old_line_at_end_is_noop(), apply_selection(), catchup_to_hunkstart_past_end_does_not_panic(), NewFromOldContent, Result, String, Vec

### Community 115 - "FAQ.md"
Cohesion: 0.17
Nodes (7): 1. <a name="credentials"></a> "Bad Credentials" Error <small><sup>[Top ▲](#table-of-contents)</sup></small>, 2. <a name="keybindings"></a> Custom key bindings <small><sup>[Top ▲](#table-of-contents)</sup></small>, 3. <a name="watcher"></a> Watching for changes <small><sup>[Top ▲](#table-of-contents)</sup></small>, <a name="table-of-contents"></a> Table of Contents, Key Config, Key Symbols, Nightlies

### Community 116 - "[0.25.0] - 2024-02-21"
Cohesion: 0.29
Nodes (7): [0.25.0] - 2024-02-21, Added, Breaking Change, Changed, commit key binding, Fixes, key binding bitflags

### Community 117 - "Spinner"
Cohesion: 0.17
Nodes (8): B, Cell, Default, Error, Result, Self, Terminal, Spinner

### Community 118 - "AsyncFetchJob"
Cohesion: 0.23
Nodes (9): AsyncFetchJob, JobState, Arc, Mutex, Notification, Option, Progress, Result (+1 more)

### Community 119 - "ShowUntrackedFilesConfig"
Cohesion: 0.29
Nodes (9): From, Self, Status, StatusItemType, StatusShow, StatusType, ChangeRef, Delta (+1 more)

### Community 120 - ".forwarder"
Cohesion: 0.27
Nodes (8): DebounceEventResult, create_watcher(), RepoWatcher, Duration, Receiver, Result, Self, Sender

### Community 121 - "6. <a name="installation"></a> Installation <small><sup>[Top ▲](#table-of-contents)</sup></small>"
Cohesion: 0.33
Nodes (6): 6. <a name="installation"></a> Installation <small><sup>[Top ▲](#table-of-contents)</sup></small>, Linux, macOS, Nightly Builds, Release Binaries, Windows

### Community 122 - "reword"
Cohesion: 0.36
Nodes (10): get_current_branch(), reword(), reword_signed(), reword_unsigned(), Branch, Option, Repository, Result (+2 more)

### Community 123 - "lib.rs"
Cohesion: 0.42
Nodes (10): init_log(), repo_init(), repo_init_bare(), repo_init_empty(), repo_init_suffix(), Option, Repository, T (+2 more)

### Community 124 - "[0.23.0] - 2023-06-19"
Cohesion: 0.40
Nodes (5): [0.23.0] - 2023-06-19, Added, Breaking Change, Changed, Fixes

### Community 125 - "[0.24.0] - 2023-08-27"
Cohesion: 0.40
Nodes (5): [0.24.0] - 2023-08-27, Added, Breaking Changes, Changed, Fixes

### Community 126 - "get_file_diff_patch"
Cohesion: 0.40
Nodes (9): get_file_diff_patch(), get_patches(), HunkLines, patch_get_hunklines(), Diff, Repository, Result, Vec (+1 more)

### Community 127 - "commitlist.rs"
Cohesion: 0.36
Nodes (6): build_commit_list_with_some_commits(), build_marked_from_indices(), test_copy_commit_none_marked(), test_copy_commit_one_marked(), test_copy_commit_random_marked(), test_copy_commit_range_marked()

### Community 128 - "[0.26.3] - 2024-06-02"
Cohesion: 0.40
Nodes (5): [0.26.3] - 2024-06-02, Added, Breaking Changes, Fixes, Theme file format

### Community 130 - "Contributing"
Cohesion: 0.40
Nodes (4): Building rusted-git, Contributing, Getting help, Getting started

### Community 131 - "7. <a name="build"></a> Build <small><sup>[Top ▲](#table-of-contents)</sup></small>"
Cohesion: 0.40
Nodes (5): 7. <a name="build"></a> Build <small><sup>[Top ▲](#table-of-contents)</sup></small>, Cargo Features, Cargo Install, Requirements, trace-libgit

### Community 132 - ".draw"
Cohesion: 0.38
Nodes (3): Frame, Rect, Self

### Community 133 - "T"
Cohesion: 0.43
Nodes (3): NotifiableMutex<T>, Self, T

### Community 134 - "blame_file"
Cohesion: 0.31
Nodes (10): blame_file(), BlameHunk, FileBlame, fixup_windows_path(), Option, Result, String, Vec (+2 more)

### Community 135 - "write_commit_file"
Cohesion: 0.27
Nodes (19): write_commit_file(), discard_lines(), Result, test_discard(), test_discard2(), test_discard3(), test_discard4(), test_discard5() (+11 more)

### Community 136 - "[0.11.0] - 2021-12-20"
Cohesion: 0.50
Nodes (4): [0.11.0] - 2021-12-20, Added, Changed, Fixed

### Community 137 - "ScopeTimeLog"
Cohesion: 0.50
Nodes (3): Drop, Instant, ScopeTimeLog

### Community 138 - "style_detail"
Cohesion: 0.60
Nodes (4): Detail, SharedTheme, Span, style_detail()

### Community 139 - "[0.12.0] - 2021-03-03"
Cohesion: 0.50
Nodes (4): [0.12.0] - 2021-03-03, Added, Breaking Change, Fixed

### Community 141 - "Error"
Cohesion: 0.50
Nodes (3): Error, PathBuf, TryFromIntError

### Community 142 - "[0.13.0] - 2021-03-15 - Happy Birthday GitUI 🥳"
Cohesion: 0.50
Nodes (4): [0.13.0] - 2021-03-15 - Happy Birthday rusted-git 🥳, Added, Changed, Fixed

### Community 143 - ".draw"
Cohesion: 0.20
Nodes (7): IntoIter, IntoIterator, &'a ItemBatch, ItemBatch, Option, Rc, Vec

### Community 144 - ".draw"
Cohesion: 0.28
Nodes (8): KeySymbols, KeySymbolsFile, Default, Option, PathBuf, Result, Self, String

### Community 149 - "[0.14.0] - 2021-04-11"
Cohesion: 0.50
Nodes (4): [0.14.0] - 2021-04-11, Added, Changed, Fixed

### Community 150 - "[0.15.0] - 2021-04-27"
Cohesion: 0.50
Nodes (4): [0.15.0] - 2021-04-27, Added, Fixed, Internal

### Community 151 - "[0.16.0] - 2021-05-28"
Cohesion: 0.50
Nodes (4): [0.16.0] - 2021-05-28, Added, Changed, Fixed

### Community 152 - "[0.16.1] - 2021-06-06"
Cohesion: 0.50
Nodes (4): [0.16.1] - 2021-06-06, Added, Fixed, Internal

### Community 153 - "[0.17.1] - 2021-09-10"
Cohesion: 0.50
Nodes (4): [0.17.1] - 2021-09-10, Added, Fixed, Key binding notes

### Community 154 - "[0.19] - 2021-12-08 - Bare Repo Support"
Cohesion: 0.50
Nodes (4): [0.19] - 2021-12-08 - Bare Repo Support, Added, Breaking Change, Fixed

### Community 155 - "[0.20] - 2022-01-25 - Tag Annotations"
Cohesion: 0.50
Nodes (4): [0.20] - 2022-01-25 - Tag Annotations, Added, Fixed, Key binding notes

### Community 156 - "[0.21.0] - 2022-08-17"
Cohesion: 0.50
Nodes (4): [0.21.0] - 2022-08-17, Added, Changed, Fixed

### Community 157 - "[0.26.0+1] - 2024-04-14"
Cohesion: 0.50
Nodes (4): [0.26.0+1] - 2024-04-14, Added, Changed, Fixes

### Community 158 - "[0.27.0] - 2025-01-14"
Cohesion: 0.50
Nodes (4): [0.27.0] - 2025-01-14, Added, Breaking Changes, Fixes

### Community 159 - "[0.28.0] - 2025-12-14"
Cohesion: 0.50
Nodes (4): [0.28.0] - 2025-12-14, Added, Changed, Fixes

### Community 160 - "[0.2.2] - 2020-05-10"
Cohesion: 0.50
Nodes (4): [0.2.2] - 2020-05-10, Added, Changed, Fixed

### Community 161 - "[0.2.5] - 2020-05-16"
Cohesion: 0.50
Nodes (4): [0.2.5] - 2020-05-16, Added, Changed, Fixed

### Community 162 - "[0.3.0] - 2020-05-20"
Cohesion: 0.50
Nodes (4): [0.3.0] - 2020-05-20, Added, Changed, Fixed

### Community 163 - "[0.4.0] - 2020-05-25"
Cohesion: 0.50
Nodes (4): [0.4.0] - 2020-05-25, Added, Changed, Fixes

### Community 164 - "[0.5.0] - 2020-06-01"
Cohesion: 0.50
Nodes (4): [0.5.0] - 2020-06-01, Added, Changed, Fixed

### Community 165 - "[0.6.0] - 2020-06-09"
Cohesion: 0.50
Nodes (4): [0.6.0] - 2020-06-09, Added, Changed, Fixed

### Community 166 - "[0.7.0] - 2020-06-15"
Cohesion: 0.50
Nodes (4): [0.7.0] - 2020-06-15, Added, Changed, Fixed

### Community 167 - "[0.8.0] - 2020-07-06"
Cohesion: 0.50
Nodes (4): [0.8.0] - 2020-07-06, Added, Changed, Fixed

### Community 168 - "[0.9.1] - 2020-07-30"
Cohesion: 0.50
Nodes (4): [0.9.1] - 2020-07-30, Added, Changed, Fixed

### Community 169 - "Unreleased"
Cohesion: 0.50
Nodes (4): Added, Changed, Fixes, Unreleased

### Community 170 - ".new"
Cohesion: 0.36
Nodes (11): ClapApp, app(), CliArgs, get_app_cache_path(), get_app_config_path(), process_cmdline(), Option, PathBuf (+3 more)

### Community 171 - "[0.10.0] - 2020-08-29"
Cohesion: 0.26
Nodes (4): DateTime, Local, Row, Vec

### Community 172 - "[0.17.0] - 2021-08-21"
Cohesion: 0.67
Nodes (3): [0.17.0] - 2021-08-21, Added, Fixed

### Community 173 - "[0.18] - 2021-10-11"
Cohesion: 0.67
Nodes (3): [0.18] - 2021-10-11, Added, Fixed

### Community 174 - "[0.20.1] - 2022-01-26"
Cohesion: 0.67
Nodes (3): [0.20.1] - 2022-01-26, Added, Fixed

### Community 175 - "[0.22.0] - 2022-11-19"
Cohesion: 0.67
Nodes (3): [0.22.0] - 2022-11-19, Added, Fixes

### Community 176 - "[0.22.1] - 2022-11-22"
Cohesion: 0.67
Nodes (3): [0.22.1] - 2022-11-22, Added, Fixes

### Community 177 - "[0.25.2] - 2024-03-22"
Cohesion: 0.67
Nodes (3): [0.25.2] - 2024-03-22, Changed, Fixes

### Community 178 - "[0.26.2] - 2024-04-17"
Cohesion: 0.67
Nodes (3): [0.26.2] - 2024-04-17, Added, Fixes

### Community 179 - "[0.28.1] - 2026-03-21"
Cohesion: 0.67
Nodes (3): [0.28.1] - 2026-03-21, Changed, Fixed

### Community 190 - "clipboard.rs"
Cohesion: 0.49
Nodes (10): copy_string(), copy_string_osc52(), copy_string_wayland(), copy_string_x(), exec_copy(), exec_copy_with_args(), is_wsl(), Result (+2 more)

### Community 191 - "FileRevOpen"
Cohesion: 0.20
Nodes (6): FileRevOpen, Event, Option, Self, Sender, String

### Community 192 - "ProgressPercent"
Cohesion: 0.36
Nodes (5): ProgressPercent, Self, test_progress_rounding(), test_progress_zero_all(), test_progress_zero_total()

### Community 198 - "[0.16.2] - 2021-07-10"
Cohesion: 0.67
Nodes (3): [0.16.2] - 2021-07-10, Added, Fixed

## Knowledge Gaps
- **176 isolated node(s):** `ScopeTimeLog<'a>`, `Direction`, `WordWrapper<'a, 'b>`, `Added`, `Changed` (+171 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `CommitId` connect `CommitId` to `ScrollType`, `strings.rs`, `BlameFilePopup`, `mod.rs`, `blame_file`, `write_commit_file`, `RefGraph`, `Revlog`, `SubmodulesListPopup`, `TagListPopup`, `.draw`, `mod.rs`, `StatusTreeComponent`, `StashList`, `TextInputComponent`, `BranchListPopup`, `CommitPopup`, `ResetPopup`, `logwalker.rs`, `stash.rs`, `tree.rs`, `LogSearchPopupPopup`, `DetailsComponent`, `RepoPath`, `AsyncGitNotification`, `CommitList`, `[0.10.0] - 2020-08-29`, `EventState`, `stage_add_file`, `debug_cmd_print`, `utils.rs`, `CompareCommitsPopup`, `AsyncDiff`, `CompareDetailsComponent`, `FilesTab`, `rebase.rs`, `RunParams`, `diff.rs`, `AsyncCommitFiles`, `get_commit_details`, `Gitui`, `Queue`, `LogEntry`, `get_commit_files`, `.get_entry_to_add`, `reword`, `commitlist.rs`?**
  _High betweenness centrality (0.200) - this node is a cross-community bridge._
- **Why does `RepoPath` connect `RepoPath` to `mod.rs`, `PushPopup`, `blame_file`, `RefGraph`, `write_commit_file`, `Status`, `SubmodulesListPopup`, `TagListPopup`, `PushTagsPopup`, `mod.rs`, `StashList`, `CommitPopup`, `ResetPopup`, `stash.rs`, `tree.rs`, `CommitId`, `AsyncGitNotification`, `repo`, `hooks.rs`, `PullPopup`, `.new`, `stage_add_file`, `CheckoutOptionPopup`, `debug_cmd_print`, `utils.rs`, `BasicAuthCredential`, `AsyncDiff`, `main.rs`, `FilesTab`, `AsyncStatus`, `AsyncCreatePrJob`, `rebase.rs`, `RunParams`, `diff.rs`, `AsyncCommitFiles`, `get_commit_details`, `get_config_string`, `get_status`, `Gitui`, `get_commit_files`, `AsyncBranchesJob`, `DiffOptions`, `add_to_ignore`, `AsyncFetchJob`, `reword`?**
  _High betweenness centrality (0.171) - this node is a cross-community bridge._
- **Why does `FileTree` connect `RevisionFilesComponent` to `.update`, `RevisionFilesPopup`, `tree.rs`?**
  _High betweenness centrality (0.084) - this node is a cross-community bridge._
- **Are the 114 inferred relationships involving `repo()` (e.g. with `.fetch()` and `.fetch_helper_with_filter()`) actually correct?**
  _`repo()` has 114 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ScopeTimeLog<'a>`, `Direction`, `WordWrapper<'a, 'b>` to the rest of the system?**
  _176 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SharedKeyConfig` be split into smaller, more focused modules?**
  _Cohesion score 0.03278236914600551 - nodes in this community are weakly interconnected._
- **Should `strings.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.03647798742138365 - nodes in this community are weakly interconnected._