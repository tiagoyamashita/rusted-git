//! SourceTree-style compact lane glyphs: one char per tip, fixed columns.

use super::graph_log::GraphCommit;
use super::CommitId;
use std::collections::{HashMap, HashSet};

/// One drawn cell in the lane column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphCell {
	/// Glyph to render.
	pub ch: char,
	/// Stable color index for this lane (cycle in the UI theme).
	pub color: usize,
}

/// Lane glyphs for a single commit row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GraphRow {
	/// Cells left-to-right (fixed width = tip count).
	pub cells: Vec<GraphCell>,
	/// Lane index of this commit's node.
	pub commit_lane: usize,
}

/// Compute SourceTree-style lanes for `commits` (newest-first).
///
/// * One fixed column per tip (HEAD / branches / tags), padded on every row
///   so a branch's commits stay on the same horizontal level.
/// * `*` marks HEAD; `●` marks other commits; merge commits keep `●` with
///   inbound connectors from other lanes.
/// * Side branches fork with `╭╮╰╯` elbows where they leave / rejoin.
#[must_use]
pub fn assign_lanes(
	commits: &[GraphCommit],
	tips: &[CommitId],
	head: Option<CommitId>,
) -> Vec<GraphRow> {
	if commits.is_empty() {
		return Vec::new();
	}

	let width = tips.len().max(1);
	let owner = assign_commit_owners(commits, tips);

	// reserved[lane] = next commit expected on that column (active line).
	let mut reserved: Vec<Option<CommitId>> = vec![None; width];
	let mut rows = Vec::with_capacity(commits.len());

	for commit in commits {
		let commit_lane = owner.get(&commit.id).copied().unwrap_or(0)
			.min(width - 1);

		// Open this tip's column when its tip first appears.
		if reserved[commit_lane] != Some(commit.id) {
			reserved[commit_lane] = Some(commit.id);
		}

		let matching: Vec<usize> = reserved
			.iter()
			.enumerate()
			.filter_map(|(i, id)| {
				(*id == Some(commit.id)).then_some(i)
			})
			.collect();

		let is_head = head == Some(commit.id);
		let is_merge = commit.parents.len() > 1;
		let mut cells = draw_row(
			&reserved,
			commit_lane,
			&matching,
			is_head,
			is_merge,
			width,
		);

		for &lane in &matching {
			reserved[lane] = None;
		}

		if let Some((first, rest)) = commit.parents.split_first() {
			let parent_lane = owner
				.get(first)
				.copied()
				.unwrap_or(commit_lane)
				.min(width - 1);

			if parent_lane == commit_lane {
				reserved[commit_lane] = Some(*first);
			} else {
				// Branch ends / forks into parent column.
				if reserved[parent_lane].is_none() {
					reserved[parent_lane] = Some(*first);
				}
				draw_elbow(
					&mut cells,
					commit_lane,
					parent_lane,
					commit_lane,
				);
			}

			// True merge: other parents pull into this commit.
			for parent in rest {
				if let Some(&pl) = owner.get(parent) {
					let pl = pl.min(width - 1);
					if reserved[pl] == Some(*parent) {
						draw_elbow(
							&mut cells,
							pl,
							commit_lane,
							pl,
						);
						reserved[pl] = None;
					} else if reserved[pl].is_none() {
						reserved[pl] = Some(*parent);
						draw_elbow(
							&mut cells,
							commit_lane,
							pl,
							commit_lane,
						);
					}
				}
			}
		}

		// Keep every row the same width so columns stay aligned.
		pad_to_width(&mut cells, width);

		rows.push(GraphRow {
			cells,
			commit_lane,
		});
	}

	rows
}

/// First tip to claim a commit via first-parent history owns that column.
fn assign_commit_owners(
	commits: &[GraphCommit],
	tips: &[CommitId],
) -> HashMap<CommitId, usize> {
	let commit_ids: HashSet<CommitId> =
		commits.iter().map(|c| c.id).collect();
	let first_parent: HashMap<CommitId, Option<CommitId>> = commits
		.iter()
		.map(|c| (c.id, c.parents.first().copied()))
		.collect();

	let mut owner = HashMap::new();

	for (lane, tip) in tips.iter().enumerate() {
		if !commit_ids.contains(tip) {
			continue;
		}
		let mut cur = Some(*tip);
		while let Some(id) = cur {
			if owner.contains_key(&id) {
				break;
			}
			if !commit_ids.contains(&id) {
				break;
			}
			owner.insert(id, lane);
			cur = first_parent.get(&id).copied().flatten();
		}
	}

	for c in commits {
		if owner.contains_key(&c.id) {
			continue;
		}
		let lane = c
			.parents
			.iter()
			.find_map(|p| owner.get(p).copied())
			.unwrap_or(0);
		owner.insert(c.id, lane);
	}

	owner
}

fn draw_row(
	reserved: &[Option<CommitId>],
	commit_lane: usize,
	matching: &[usize],
	is_head: bool,
	_is_merge: bool,
	width: usize,
) -> Vec<GraphCell> {
	let mut cells = Vec::with_capacity(width);

	for i in 0..width {
		let ch = if i == commit_lane {
			if is_head {
				'*'
			} else {
				'●'
			}
		} else if matching.contains(&i) {
			// Extra tip landing on same commit (merged tip).
			if i > commit_lane {
				'╯'
			} else {
				'╰'
			}
		} else if reserved.get(i).is_some_and(Option::is_some) {
			'│'
		} else {
			' '
		};
		cells.push(GraphCell { ch, color: i });
	}

	// Connect secondary matching lanes into the commit node.
	for &lane in matching.iter().skip(1) {
		fill_between(&mut cells, commit_lane, lane, lane);
		if let Some(cell) = cells.get_mut(lane) {
			cell.ch = if lane > commit_lane { '╯' } else { '╰' };
			cell.color = lane;
		}
	}

	cells
}

fn draw_elbow(
	cells: &mut [GraphCell],
	from: usize,
	to: usize,
	color: usize,
) {
	if from == to || cells.is_empty() {
		return;
	}
	fill_between(cells, from, to, color);

	if let Some(cell) = cells.get_mut(to) {
		if cell.ch == ' ' || cell.ch == '─' || cell.ch == '│' {
			cell.ch = if to > from { '╮' } else { '╭' };
			cell.color = color;
		}
	}
}

fn fill_between(
	cells: &mut [GraphCell],
	from: usize,
	to: usize,
	color: usize,
) {
	let left = from.min(to);
	let right = from.max(to);
	for i in left..=right {
		if i >= cells.len() || i == from {
			continue;
		}
		if cells[i].ch == ' ' || cells[i].ch == '─' {
			cells[i] = GraphCell { ch: '─', color };
		}
	}
}

fn pad_to_width(cells: &mut Vec<GraphCell>, width: usize) {
	while cells.len() < width {
		cells.push(GraphCell {
			ch: ' ',
			color: 0,
		});
	}
	if cells.len() > width {
		cells.truncate(width);
	}
}

/// Render a row as a plain string (tests / debugging).
#[cfg(test)]
#[must_use]
pub fn row_to_string(row: &GraphRow) -> String {
	row.cells.iter().map(|c| c.ch).collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use git2::Oid;

	fn cid(n: u8) -> CommitId {
		let mut bytes = [0_u8; 20];
		bytes[19] = n;
		CommitId::new(Oid::from_bytes(&bytes).unwrap())
	}

	#[test]
	fn test_linear_history_single_lane() {
		let a = cid(1);
		let b = cid(2);
		let c = cid(3);
		let commits = vec![
			GraphCommit {
				id: a,
				parents: vec![b],
			},
			GraphCommit {
				id: b,
				parents: vec![c],
			},
			GraphCommit {
				id: c,
				parents: vec![],
			},
		];
		let rows = assign_lanes(&commits, &[a], Some(a));
		assert_eq!(rows.len(), 3);
		assert_eq!(row_to_string(&rows[0]), "*");
		assert_eq!(row_to_string(&rows[1]), "●");
		assert_eq!(row_to_string(&rows[2]), "●");
		assert_eq!(rows[0].commit_lane, 0);
	}

	#[test]
	fn test_diverged_branches_fixed_width() {
		let tip_a = cid(1);
		let tip_b = cid(2);
		let base = cid(3);
		let commits = vec![
			GraphCommit {
				id: tip_a,
				parents: vec![base],
			},
			GraphCommit {
				id: tip_b,
				parents: vec![base],
			},
			GraphCommit {
				id: base,
				parents: vec![],
			},
		];
		let rows =
			assign_lanes(&commits, &[tip_a, tip_b], Some(tip_a));
		assert_eq!(rows.len(), 3);
		// Every row padded to 2 columns.
		for r in &rows {
			assert_eq!(r.cells.len(), 2, "{}", row_to_string(r));
		}
		assert_eq!(rows[0].commit_lane, 0);
		assert_eq!(row_to_string(&rows[0]).chars().nth(0), Some('*'));
		assert_eq!(rows[1].commit_lane, 1);
		assert_eq!(row_to_string(&rows[1]).chars().nth(1), Some('●'));
		assert_eq!(rows[2].commit_lane, 0);
	}

	#[test]
	fn test_feature_forks_aligned_columns() {
		// main: root - mid - main_tip(HEAD)
		// feature:       mid - feat_tip
		let root = cid(1);
		let mid = cid(2);
		let main_tip = cid(3);
		let feat_tip = cid(4);
		let commits = vec![
			GraphCommit {
				id: feat_tip,
				parents: vec![mid],
			},
			GraphCommit {
				id: main_tip,
				parents: vec![mid],
			},
			GraphCommit {
				id: mid,
				parents: vec![root],
			},
			GraphCommit {
				id: root,
				parents: vec![],
			},
		];
		let rows =
			assign_lanes(&commits, &[main_tip, feat_tip], Some(main_tip));

		for r in &rows {
			assert_eq!(
				r.cells.len(),
				2,
				"fixed pad: {}",
				row_to_string(r)
			);
		}
		assert_eq!(rows[0].commit_lane, 1);
		assert_eq!(rows[1].commit_lane, 0);
		assert_eq!(row_to_string(&rows[1]).chars().nth(0), Some('*'));
		assert_eq!(rows[2].commit_lane, 0);
		assert_eq!(rows[3].commit_lane, 0);
	}
}
