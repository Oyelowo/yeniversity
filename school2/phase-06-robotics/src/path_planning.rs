//! Path planning: A* on a 2-D grid.

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Grid = Vec<Vec<bool>>; // true = free, false = obstacle

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

/// A* search on a grid. Returns path from `start` to `goal` (inclusive), or None.
pub fn astar(grid: &Grid, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut open: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
    let mut g: HashMap<(usize, usize), usize> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    g.insert(start, 0);
    open.push(Reverse((heuristic(start, goal), start)));

    while let Some(Reverse((_, current))) = open.pop() {
        if current == goal {
            let mut path = vec![goal];
            let mut cur = goal;
            while let Some(&prev) = came_from.get(&cur) {
                path.push(prev);
                cur = prev;
            }
            path.reverse();
            return Some(path);
        }
        let (r, c) = current;
        for (dr, dc) in [(!0, 0), (1, 0), (0, !0), (0usize, 1usize)] {
            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
            if nr >= rows || nc >= cols || !grid[nr][nc] { continue; }
            let ng = g[&current] + 1;
            if ng < *g.get(&(nr, nc)).unwrap_or(&usize::MAX) {
                g.insert((nr, nc), ng);
                came_from.insert((nr, nc), current);
                open.push(Reverse((ng + heuristic((nr, nc), goal), (nr, nc))));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_path_found() {
        let grid = vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
        let path = astar(&grid, (0, 0), (2, 2)).expect("path should exist");
        assert_eq!(*path.first().unwrap(), (0, 0));
        assert_eq!(*path.last().unwrap(), (2, 2));
    }

    #[test]
    fn blocked_goal_returns_none() {
        let grid = vec![
            vec![true, false],
            vec![false, false],
        ];
        assert!(astar(&grid, (0, 0), (1, 1)).is_none());
    }
}
