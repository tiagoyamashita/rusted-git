//! SourceTree-style compact lanes: branch lines run until they rejoin the parent.

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
/// Each tip keeps a fixed column. A side-branch line stays in its column
/// (`│`) until it reaches the commit it branched from on the original
/// lane — that row shows the join (`╯`/`╰`) into the parent. HEAD is `*`.
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
	let mut reserved: Vec<Option<CommitId>> = vec![None; width];
	let mut rows = Vec::with_capacity(commits.len());

	for commit in commits {
		let commit_lane = owner
			.get(&commit.id)
			.copied()
			.unwrap_or(0)
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
		let mut cells =
			draw_row(&reserved, commit_lane, &matching, is_head, width);

		// Lanes that meet here join into the commit's column.
		for &lane in &matching {
			if lane != commit_lane {
				draw_join(&mut cells, lane, commit_lane);
			}
			reserved[lane] = None;
		}

		// Keep each active lane flowing to its first parent — even when that
		// parent lives on another tip's column. The join is drawn when we
		// later reach that parent (SourceTree: line emerges from original).
		if let Some((first, rest)) = commit.parents.split_first() {
			reserved[commit_lane] = Some(*first);

			for parent in rest {
				// Merge commits: keep other parents alive on their lanes.
				if let Some(&pl) = owner.get(parent) {
					let pl = pl.min(width - 1);
					if reserved[pl].is_none() {
						reserved[pl] = Some(*parent);
						draw_join(&mut cells, pl, commit_lane);
					}
				} else {
					// Unknown parent lane: reuse a free column if any.
					if let Some(free) =
						reserved.iter().position(Option::is_none)
					{
						reserved[free] = Some(*parent);
						draw_join(&mut cells, free, commit_lane);
					}
				}
			}
		}

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
			// Placeholder; draw_join overwrites with elbow.
			'│'
		} else if reserved.get(i).is_some_and(Option::is_some) {
			'│'
		} else {
			' '
		};
		cells.push(GraphCell { ch, color: i });
	}

	cells
}

/// Draw a one-char join from `from` into `to` (the original / target lane).
fn draw_join(cells: &mut [GraphCell], from: usize, to: usize) {
	if from == to || from >= cells.len() || to >= cells.len() {
		return;
	}

	// Horizontal bridge across empty columns between the two lanes.
	let left = from.min(to);
	let right = from.max(to);
	for i in left..=right {
		if i == from || i == to {
			continue;
		}
		if cells[i].ch == ' ' {
			cells[i] = GraphCell {
				ch: '─',
				color: from,
			};
		}
	}

	// Elbow on the side branch column — line merges into the original.
	if let Some(cell) = cells.get_mut(from) {
		cell.ch = if from > to { '╯' } else { '╰' };
		cell.color = from;
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
		assert_eq!(row_to_string(&rows[0]), "*");
		assert_eq!(row_to_string(&rows[1]), "●");
		assert_eq!(row_to_string(&rows[2]), "●");
	}

	#[test]
	fn test_side_branch_line_runs_until_parent() {
		// main: root - mid - main_tip(HEAD)
		// feature:       mid - feat_tip
		// Newest-first graph should keep feature's │ beside main until mid:
		//   ●   feat
		//  *│   main HEAD
		//  ●╯   mid  (feature merges into original)
		//  ●    root
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
		let rows = assign_lanes(
			&commits,
			&[main_tip, feat_tip],
			Some(main_tip),
		);

		assert_eq!(row_to_string(&rows[0]), " ●", "feat tip");
		assert_eq!(
			row_to_string(&rows[1]),
			"*│",
			"main tip with feature line still running"
		);
		let mid_row = row_to_string(&rows[2]);
		assert_eq!(mid_row.chars().nth(0), Some('●'));
		assert!(
			mid_row.chars().nth(1) == Some('╯')
				|| mid_row.chars().nth(1) == Some('╰'),
			"feature joins original at mid, got {mid_row}"
		);
		assert_eq!(row_to_string(&rows[3]), "● ");
	}

	#[test]
	fn test_diverged_tips_join_at_base() {
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
		assert_eq!(row_to_string(&rows[0]), "* ");
		assert_eq!(row_to_string(&rows[1]), "│●");
		let base_row = row_to_string(&rows[2]);
		assert_eq!(base_row.chars().nth(0), Some('●'));
		assert!(
			matches!(base_row.chars().nth(1), Some('╯' | '╰')),
			"got {base_row}"
		);
	}
}
