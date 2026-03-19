//! Reinforcement learning stubs: Q-table, epsilon-greedy, Bellman update.

/// Epsilon-greedy action selection from a Q-row.
pub fn epsilon_greedy(q_row: &[f64], epsilon: f64, rng_rand: f64, rng_action: f64) -> usize {
    if rng_rand < epsilon {
        (rng_action * q_row.len() as f64) as usize
    } else {
        q_row.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0
    }
}

/// Bellman Q-update: Q(s,a) ← Q(s,a) + α[r + γ max_a'Q(s',a') − Q(s,a)]
pub fn q_update(q_sa: f64, reward: f64, gamma: f64, alpha: f64, q_next_max: f64) -> f64 {
    q_sa + alpha * (reward + gamma * q_next_max - q_sa)
}
