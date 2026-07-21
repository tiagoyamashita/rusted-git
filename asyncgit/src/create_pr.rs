//! Create a GitHub pull request via the `gh` CLI.

use crate::{
	asyncjob::{AsyncJob, RunParams},
	error::{Error, Result},
	sync::{utils::repo_work_dir, RepoPath},
	AsyncGitNotification, ProgressPercent,
};
use std::{
	process::Command,
	sync::{Arc, Mutex},
};

/// Inputs for `gh pr create`.
#[derive(Debug, Clone)]
pub struct CreatePrRequest {
	/// Base branch (e.g. `master` / `main`).
	pub base: String,
	/// Head branch to merge from.
	pub head: String,
	/// PR title.
	pub title: String,
	/// PR body.
	pub body: String,
}

/// Summary of an open GitHub pull request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenPrInfo {
	/// Pull request number.
	pub number: u64,
	/// Source branch name.
	pub head: String,
	/// Target branch name.
	pub base: String,
	/// Pull request URL.
	pub url: String,
}

enum JobState {
	Request(CreatePrRequest),
	Response(Result<String>),
}

enum ListJobState {
	Request,
	Response(Result<Vec<OpenPrInfo>>),
}

/// Runs `gh pr create` in a background job.
#[derive(Clone)]
pub struct AsyncCreatePrJob {
	state: Arc<Mutex<Option<JobState>>>,
	repo: RepoPath,
}

impl AsyncCreatePrJob {
	///
	pub fn new(repo: RepoPath, request: CreatePrRequest) -> Self {
		Self {
			repo,
			state: Arc::new(Mutex::new(Some(JobState::Request(
				request,
			)))),
		}
	}

	/// Take the finished result, if any.
	pub fn result(&self) -> Option<Result<String>> {
		self.state.lock().ok().and_then(|mut state| {
			match state.take() {
				Some(JobState::Response(result)) => Some(result),
				other => {
					*state = other;
					None
				}
			}
		})
	}
}

impl AsyncJob for AsyncCreatePrJob {
	type Notification = AsyncGitNotification;
	type Progress = ProgressPercent;

	fn run(
		&mut self,
		_params: RunParams<Self::Notification, Self::Progress>,
	) -> Result<Self::Notification> {
		if let Ok(mut state) = self.state.lock() {
			*state = state.take().map(|state| match state {
				JobState::Request(request) => JobState::Response(
					create_pull_request(&self.repo, &request),
				),
				JobState::Response(result) => {
					JobState::Response(result)
				}
			});
		}

		Ok(AsyncGitNotification::CreatePr)
	}
}

/// Lists open GitHub pull requests in a background job.
#[derive(Clone)]
pub struct AsyncListOpenPrsJob {
	state: Arc<Mutex<Option<ListJobState>>>,
	repo: RepoPath,
}

impl AsyncListOpenPrsJob {
	/// Create an open-PR listing job.
	pub fn new(repo: RepoPath) -> Self {
		Self {
			repo,
			state: Arc::new(Mutex::new(Some(ListJobState::Request))),
		}
	}

	/// Take the finished result, if any.
	pub fn result(&self) -> Option<Result<Vec<OpenPrInfo>>> {
		self.state.lock().ok().and_then(|mut state| {
			match state.take() {
				Some(ListJobState::Response(result)) => Some(result),
				other => {
					*state = other;
					None
				}
			}
		})
	}
}

impl AsyncJob for AsyncListOpenPrsJob {
	type Notification = AsyncGitNotification;
	type Progress = ProgressPercent;

	fn run(
		&mut self,
		_params: RunParams<Self::Notification, Self::Progress>,
	) -> Result<Self::Notification> {
		if let Ok(mut state) = self.state.lock() {
			*state = state.take().map(|state| match state {
				ListJobState::Request => ListJobState::Response(
					list_open_pull_requests(&self.repo),
				),
				ListJobState::Response(result) => {
					ListJobState::Response(result)
				}
			});
		}

		Ok(AsyncGitNotification::OpenPrs)
	}
}

fn list_open_pull_requests(
	repo: &RepoPath,
) -> Result<Vec<OpenPrInfo>> {
	let work_dir = repo_work_dir(repo)?;
	let output = Command::new("gh")
		.args([
			"pr",
			"list",
			"--state",
			"open",
			"--json",
			"number,headRefName,baseRefName,url",
			"--template",
			"{{range .}}{{.number}}\t{{.headRefName}}\t{{.baseRefName}}\t{{.url}}\n{{end}}",
		])
		.current_dir(&work_dir)
		.output()
		.map_err(|e| {
			Error::Generic(format!(
				"failed to run `gh` (is it installed and on PATH?): {e}"
			))
		})?;

	if !output.status.success() {
		let stderr = String::from_utf8_lossy(&output.stderr)
			.trim()
			.to_string();
		return Err(Error::Generic(if stderr.is_empty() {
			format!("gh pr list failed with status {}", output.status)
		} else {
			stderr
		}));
	}

	let stdout = String::from_utf8_lossy(&output.stdout);
	stdout
		.lines()
		.filter(|line| !line.trim().is_empty())
		.map(|line| {
			let mut fields = line.splitn(4, '\t');
			let number = fields
				.next()
				.and_then(|value| value.parse::<u64>().ok())
				.ok_or_else(|| {
					Error::Generic(format!(
						"invalid PR number returned by gh: {line}"
					))
				})?;
			let head = fields.next().ok_or_else(|| {
				Error::Generic(format!(
					"missing PR head returned by gh: {line}"
				))
			})?;
			let base = fields.next().ok_or_else(|| {
				Error::Generic(format!(
					"missing PR base returned by gh: {line}"
				))
			})?;
			let url = fields.next().ok_or_else(|| {
				Error::Generic(format!(
					"missing PR URL returned by gh: {line}"
				))
			})?;
			Ok(OpenPrInfo {
				number,
				head: head.to_string(),
				base: base.to_string(),
				url: url.to_string(),
			})
		})
		.collect()
}

fn create_pull_request(
	repo: &RepoPath,
	request: &CreatePrRequest,
) -> Result<String> {
	let work_dir = repo_work_dir(repo)?;

	if request.title.trim().is_empty() {
		return Err(Error::Generic(
			"pull request title must not be empty".into(),
		));
	}
	if request.head.trim().is_empty() {
		return Err(Error::Generic(
			"head branch must not be empty".into(),
		));
	}
	if request.base.trim().is_empty() {
		return Err(Error::Generic(
			"base branch must not be empty".into(),
		));
	}

	let output = Command::new("gh")
		.args([
			"pr",
			"create",
			"--base",
			request.base.trim(),
			"--head",
			request.head.trim(),
			"--title",
			request.title.trim(),
			"--body",
			&request.body,
		])
		.current_dir(&work_dir)
		.output()
		.map_err(|e| {
			Error::Generic(format!(
				"failed to run `gh` (is it installed and on PATH?): {e}"
			))
		})?;

	let stdout =
		String::from_utf8_lossy(&output.stdout).trim().to_string();
	let stderr =
		String::from_utf8_lossy(&output.stderr).trim().to_string();

	if output.status.success() {
		if stdout.is_empty() {
			Ok("pull request created".into())
		} else {
			Ok(stdout)
		}
	} else {
		let msg = if stderr.is_empty() {
			format!(
				"gh pr create failed with status {}",
				output.status
			)
		} else {
			stderr
		};
		Err(Error::Generic(msg))
	}
}
