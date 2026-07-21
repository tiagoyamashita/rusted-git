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

enum JobState {
	Request(CreatePrRequest),
	Response(Result<String>),
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
				JobState::Request(request) => {
					JobState::Response(create_pull_request(
						&self.repo, &request,
					))
				}
				JobState::Response(result) => {
					JobState::Response(result)
				}
			});
		}

		Ok(AsyncGitNotification::CreatePr)
	}
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

	let stdout = String::from_utf8_lossy(&output.stdout)
		.trim()
		.to_string();
	let stderr = String::from_utf8_lossy(&output.stderr)
		.trim()
		.to_string();

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
