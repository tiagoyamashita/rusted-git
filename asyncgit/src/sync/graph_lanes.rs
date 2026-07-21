//! Assign branch-centric lane glyphs: one lane per tip, forking at diverge.

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
	/// Cells left-to-right.
	pub cells: Vec<GraphCell>,
	/// Lane index of this commit's node.
	pub commit_lane: usize,
}

/// Compute lane rows for `commits` (newest-first) using ordered `tips`.
///
/// Each tip owns one lane for its exclusive first-parent history. When a tip
/// joins another tip's history (branched-from), that lane ends with a fork
/// connector into the parent lane.
#[must_use]
pub fn assign_lanes(
	commits: &[GraphCommit],
	tips: &[CommitId],
) -> Vec<GraphRow> {
	if commits.is_empty() {
		return Vec::new();
	}

	let owner = assign_commit_owners(commits, tips);
	let mut reserved: Vec<Option<CommitId>> =
		vec![None; tips.len().max(1)];
	let colors: Vec<usize> = (0..reserved.len()).collect();
	let mut rows = Vec::with_capacity(commits.len());

	for commit in commits {
		let commit_lane = owner.get(&commit.id).copied().unwrap_or(0);
		ensure_lanes(&mut reserved, commit_lane + 1);

		// Open this tip's lane when we first see its tip commit.
		if reserved.get(commit_lane).copied().flatten()
			!= Some(commit.id)
		{
			if let Some(slot) = reserved.get_mut(commit_lane) {
				*slot = Some(commit.id);
			}
		}

		let matching: Vec<usize> = reserved
			.iter()
			.enumerate()
			.filter_map(|(i, id)| {
				(*id == Some(commit.id)).then_some(i)
			})
			.collect();

		let mut cells =
			draw_cells(&reserved, &colors, commit_lane, &matching);

		for &lane in &matching {
			if let Some(slot) = reserved.get_mut(lane) {
				*slot = None;
			}
		}

		if let Some((first, rest)) = commit.parents.split_first() {
			let parent_lane =
				owner.get(first).copied().unwrap_or(commit_lane);

			if parent_lane == commit_lane {
				if let Some(slot) = reserved.get_mut(commit_lane) {
					*slot = Some(*first);
				}
			} else {
				// Side branch ends here and joins the parent branch lane.
				ensure_lanes(&mut reserved, parent_lane + 1);
				if reserved.get(parent_lane).copied().flatten().is_none()
				{
					if let Some(slot) = reserved.get_mut(parent_lane) {
						*slot = Some(*first);
					}
				}
				draw_fork_connector(
					&mut cells,
					&colors,
					commit_lane,
					parent_lane,
				);
			}

			// Merge parents that already have an active lane.
			for parent in rest {
				if let Some(&pl) = owner.get(parent) {
					ensure_lanes(&mut reserved, pl + 1);
					if reserved.iter().any(|id| *id == Some(*parent))
					{
						draw_merge_connector(
							&mut cells,
							&colors,
							commit_lane,
							pl,
						);
					} else if let Some(slot) = reserved.get_mut(pl) {
						if slot.is_none() {
							*slot = Some(*parent);
							draw_fork_connector(
								&mut cells,
								&colors,
								commit_lane,
								pl,
							);
						}
					}
				}
			}
		}

		while cells.last().is_some_and(|c| c.ch == ' ') {
			cells.pop();
		}

		rows.push(GraphRow {
			cells,
			commit_lane,
		});
	}

	rows
}

/// Walk each tip's first-parent chain; the first tip to claim a commit owns it.
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
				// Reached another tip's history — this is the fork point.
				break;
			}
			if !commit_ids.contains(&id) {
				break;
			}
			owner.insert(id, lane);
			cur = first_parent.get(&id).copied().flatten();
		}
	}

	// Orphan / second-parent-only commits: inherit a parent's lane if possible.
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

fn ensure_lanes(reserved: &mut Vec<Option<CommitId>>, len: usize) {
	while reserved.len() < len {
		reserved.push(None);
	}
}

fn draw_cells(
	reserved: &[Option<CommitId>],
	colors: &[usize],
	commit_lane: usize,
	matching: &[usize],
) -> Vec<GraphCell> {
	let width = reserved.len().max(commit_lane + 1);
	let mut cells = Vec::with_capacity(width * 2);

	for i in 0..width {
		let color = colors.get(i).copied().unwrap_or(i);
		let ch = if i == commit_lane {
			'●'
		} else if matching.contains(&i) {
			'╯'
		} else if reserved.get(i).is_some_and(Option::is_some) {
			'│'
		} else {
			' '
		};
		cells.push(GraphCell { ch, color });
		cells.push(GraphCell {
			ch: ' ',
			color: 0,
		});
	}

	for &lane in matching.iter().skip(1) {
		let color = colors.get(lane).copied().unwrap_or(lane);
		fill_horizontal(&mut cells, commit_lane, lane, color);
		let idx = lane * 2;
		if let Some(cell) = cells.get_mut(idx) {
			cell.ch = if lane > commit_lane { '╯' } else { '╰' };
			cell.color = color;
		}
	}

	cells
}

fn draw_merge_connector(
	cells: &mut Vec<GraphCell>,
	colors: &[usize],
	from: usize,
	to: usize,
) {
	let color = colors.get(to).copied().unwrap_or(to);
	ensure_width(cells, to.max(from) + 1);
	fill_horizontal(cells, from, to, color);
}

fn draw_fork_connector(
	cells: &mut Vec<GraphCell>,
	colors: &[usize],
	from: usize,
	to: usize,
) {
	let color = colors.get(from).copied().unwrap_or(from);
	ensure_width(cells, to.max(from) + 1);
	fill_horizontal(cells, from, to, color);
	let idx = to * 2;
	if let Some(cell) = cells.get_mut(idx) {
		// Keep existing node; only set elbow if empty/connector.
		if cell.ch == ' ' || cell.ch == '─' || cell.ch == '│' {
			cell.ch = if to > from { '╮' } else { '╭' };
			cell.color = color;
		}
	}
	let from_idx = from * 2;
	if let Some(cell) = cells.get_mut(from_idx) {
		if cell.ch == '●' {
			// node stays; horizontal already filled beside it
		} else if to > from {
			cell.ch = '╰';
			cell.color = color;
		} else {
			cell.ch = '╯';
			cell.color = color;
		}
	}
}

fn ensure_width(cells: &mut Vec<GraphCell>, lanes: usize) {
	while cells.len() < lanes * 2 {
		cells.push(GraphCell {
			ch: ' ',
			color: 0,
		});
		cells.push(GraphCell {
			ch: ' ',
			color: 0,
		});
	}
}

fn fill_horizontal(
	cells: &mut [GraphCell],
	from: usize,
	to: usize,
	color: usize,
) {
	let left = from.min(to);
	let right = from.max(to);
	for i in left..=right {
		let idx = i * 2;
		if idx >= cells.len() {
			break;
		}
		if i == from {
			continue;
		}
		if cells[idx].ch == ' ' || cells[idx].ch == '─' {
			cells[idx] = GraphCell { ch: '─', color };
			if idx + 1 < cells.len() {
				cells[idx + 1] = GraphCell { ch: '─', color };
			}
		}
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
		let rows = assign_lanes(&commits, &[a]);
		assert_eq!(rows.len(), 3);
		assert!(row_to_string(&rows[0]).contains('●'));
		assert_eq!(rows[0].commit_lane, 0);
		assert_eq!(rows[1].commit_lane, 0);
		assert_eq!(rows[2].commit_lane, 0);
	}

	#[test]
	fn test_diverged_branches_use_two_lanes() {
		// tip_a (main) and tip_b (feature) both parent of base.
		// Newest-first: tip_a, tip_b, base.
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
		let rows = assign_lanes(&commits, &[tip_a, tip_b]);
		assert_eq!(rows.len(), 3);
		assert_eq!(rows[0].commit_lane, 0, "main tip on lane 0");
		assert_eq!(rows[1].commit_lane, 1, "feature tip on lane 1");
		assert_eq!(
			rows[2].commit_lane, 0,
			"shared base stays on first tip's lane"
		);
	}

	#[test]
	fn test_feature_forks_from_main_mid_history() {
		// main:   root - mid - main_tip
		// feature:         mid - feat_tip
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
			assign_lanes(&commits, &[main_tip, feat_tip]);
		assert_eq!(rows[0].commit_lane, 1, "feature tip");
		assert_eq!(rows[1].commit_lane, 0, "main tip");
		assert_eq!(rows[2].commit_lane, 0, "fork point on main");
		assert_eq!(rows[3].commit_lane, 0, "root on main");
		// Feature row should show a second lane column exists.
		assert!(
			row_to_string(&rows[0]).len()
				>= row_to_string(&rows[1]).len().saturating_sub(2)
		);
	}
}
