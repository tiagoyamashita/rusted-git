//! Compact SourceTree-style lanes: branch lines run until they rejoin the parent.
//!
//! Columns are allocated dynamically and reused after a join, so width tracks
//! peak concurrent branches instead of total tip count.

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
	/// Cells left-to-right (width = peak concurrent lanes so far, trimmed later).
	pub cells: Vec<GraphCell>,
	/// Lane index of this commit's node.
	pub commit_lane: usize,
}

/// Compute compact lanes for `commits` (newest-first).
///
/// Each active branch keeps a column (`│`) until it reaches the commit it
/// branched from — that row shows the join (`╯`/`╰`) into the parent. Freed
/// columns are reused. HEAD is `*`. `tips` only biases ownership/color (earlier
/// tips preferred); they do not reserve permanent columns.
#[must_use]
pub fn assign_lanes(
	commits: &[GraphCommit],
	tips: &[CommitId],
	head: Option<CommitId>,
) -> Vec<GraphRow> {
	if commits.is_empty() {
		return Vec::new();
	}

	let owner = assign_commit_owners(commits, tips);
	let mut reserved: Vec<Option<CommitId>> = Vec::new();
	let mut lane_color: Vec<usize> = Vec::new();
	let mut rows = Vec::with_capacity(commits.len());

	for commit in commits {
		let tip_color = owner.get(&commit.id).copied().unwrap_or(0);

		let matching: Vec<usize> = reserved
			.iter()
			.enumerate()
			.filter_map(|(i, id)| {
				(*id == Some(commit.id)).then_some(i)
			})
			.collect();

		let commit_lane = if matching.is_empty() {
			let lane = open_lane(&mut reserved, &mut lane_color, tip_color);
			reserved[lane] = Some(commit.id);
			lane
		} else {
			// Prefer the leftmost matching lane; bias toward this tip's color.
			matching
				.iter()
				.copied()
				.min_by_key(|&lane| {
					let color_mismatch =
						usize::from(lane_color.get(lane).copied() != Some(tip_color));
					(color_mismatch, lane)
				})
				.unwrap_or(0)
		};

		let is_head = head == Some(commit.id);
		let mut cells =
			draw_row(&reserved, &lane_color, commit_lane, &matching, is_head);

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
			lane_color[commit_lane] = tip_color;

			for parent in rest {
				if let Some(existing) = reserved
					.iter()
					.position(|id| *id == Some(*parent))
				{
					ensure_cells_width(
						&mut cells,
						reserved.len(),
						&lane_color,
					);
					draw_join(&mut cells, existing, commit_lane);
					continue;
				}
				let parent_color =
					owner.get(parent).copied().unwrap_or(tip_color);
				let pl = open_lane(
					&mut reserved,
					&mut lane_color,
					parent_color,
				);
				reserved[pl] = Some(*parent);
				ensure_cells_width(
					&mut cells,
					reserved.len(),
					&lane_color,
				);
				draw_join(&mut cells, pl, commit_lane);
			}
		}

		rows.push(GraphRow {
			cells,
			commit_lane,
		});
	}

	normalize_row_widths(&mut rows);
	rows
}

/// First tip to claim a commit via first-parent history owns that color index.
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

fn open_lane(
	reserved: &mut Vec<Option<CommitId>>,
	lane_color: &mut Vec<usize>,
	color: usize,
) -> usize {
	if let Some(i) = reserved.iter().position(Option::is_none) {
		lane_color[i] = color;
		return i;
	}
	reserved.push(None);
	lane_color.push(color);
	reserved.len() - 1
}

fn draw_row(
	reserved: &[Option<CommitId>],
	lane_color: &[usize],
	commit_lane: usize,
	matching: &[usize],
	is_head: bool,
) -> Vec<GraphCell> {
	let width = reserved.len().max(commit_lane + 1);
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
		cells.push(GraphCell {
			ch,
			color: lane_color.get(i).copied().unwrap_or(i),
		});
	}

	cells
}

fn ensure_cells_width(
	cells: &mut Vec<GraphCell>,
	width: usize,
	lane_color: &[usize],
) {
	while cells.len() < width {
		let i = cells.len();
		cells.push(GraphCell {
			ch: ' ',
			color: lane_color.get(i).copied().unwrap_or(i),
		});
	}
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

/// Pad every row to the same width (peak used columns) so the hash column aligns.
fn normalize_row_widths(rows: &mut [GraphRow]) {
	let max_w = rows
		.iter()
		.map(|r| {
			r.cells
				.iter()
				.rposition(|c| c.ch != ' ')
				.map_or(0, |i| i + 1)
		})
		.max()
		.unwrap_or(0)
		.max(1);

	for row in rows {
		while row.cells.len() < max_w {
			row.cells.push(GraphCell {
				ch: ' ',
				color: 0,
			});
		}
		row.cells.truncate(max_w);
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
		// Newest-first: feature opens leftmost, main opens beside it.
		//   ●│  or ●   feat
		//   │*       main HEAD
		//   ●╯       mid  (feature joins)
		//   ●        root
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

		assert_eq!(
			row_to_string(&rows[0]).chars().filter(|&c| c != ' ').count(),
			1,
			"feat tip is a single node: {}",
			row_to_string(&rows[0])
		);
		assert!(
			row_to_string(&rows[1]).contains('*'),
			"main tip is HEAD: {}",
			row_to_string(&rows[1])
		);
		assert!(
			row_to_string(&rows[1]).contains('│'),
			"feature line still running beside main: {}",
			row_to_string(&rows[1])
		);
		let mid_row = row_to_string(&rows[2]);
		assert!(
			mid_row.contains('●')
				&& (mid_row.contains('╯') || mid_row.contains('╰')),
			"feature joins original at mid, got {mid_row}"
		);
		assert_eq!(
			row_to_string(&rows[3]).chars().filter(|&c| c != ' ').count(),
			1
		);
		// Never wider than peak concurrent (2).
		assert!(
			rows.iter().all(|r| r.cells.len() <= 2),
			"graph wider than 2 lanes"
		);
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
		assert!(row_to_string(&rows[0]).contains('*'));
		assert!(row_to_string(&rows[1]).contains('●'));
		assert!(row_to_string(&rows[1]).contains('│'));
		let base_row = row_to_string(&rows[2]);
		assert!(
			base_row.contains('●')
				&& matches!(
					base_row.chars().find(|&c| c == '╯' || c == '╰'),
					Some(_)
				),
			"got {base_row}"
		);
		assert!(rows.iter().all(|r| r.cells.len() <= 2));
	}

	#[test]
	fn test_many_tips_reuse_lanes_not_tip_count() {
		// Five tips that all share one base — peak concurrent is 5 at the tips,
		// but we must not pad to a larger artificial width, and after joins
		// history collapses.
		let base = cid(1);
		let tips: Vec<_> = (2..=6).map(cid).collect();
		let mut commits: Vec<_> = tips
			.iter()
			.rev()
			.map(|&id| GraphCommit {
				id,
				parents: vec![base],
			})
			.collect();
		commits.push(GraphCommit {
			id: base,
			parents: vec![],
		});

		let rows = assign_lanes(&commits, &tips, Some(tips[0]));
		assert!(
			rows.iter().all(|r| r.cells.len() <= tips.len()),
			"width {} exceeds tip count",
			rows[0].cells.len()
		);
		// Peak concurrent = number of tips before they join.
		assert_eq!(rows[0].cells.len(), tips.len());
	}
}
