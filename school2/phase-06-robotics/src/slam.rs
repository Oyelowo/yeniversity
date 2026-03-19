//! SLAM stubs — occupancy grid and EKF-SLAM outlines.

/// Simple 2-D occupancy grid (log-odds representation).
pub struct OccupancyGrid {
    pub rows: usize,
    pub cols: usize,
    pub log_odds: Vec<Vec<f64>>,
}

impl OccupancyGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols, log_odds: vec![vec![0.0; cols]; rows] }
    }

    /// Update a cell with a hit (occupied) observation.
    pub fn observe_hit(&mut self, r: usize, c: usize) {
        self.log_odds[r][c] += 0.85;
    }

    /// Update a cell with a miss (free) observation.
    pub fn observe_miss(&mut self, r: usize, c: usize) {
        self.log_odds[r][c] -= 0.4;
    }

    /// Return occupancy probability for a cell.
    pub fn probability(&self, r: usize, c: usize) -> f64 {
        let l = self.log_odds[r][c];
        1.0 / (1.0 + (-l).exp())
    }
}
