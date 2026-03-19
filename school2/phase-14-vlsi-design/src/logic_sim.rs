//! Gate-level logic simulator: nodes and events.


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Logic { Low, High, X }

#[derive(Debug)]
pub enum Gate { And, Or, Nand, Nor, Xor, Inv }

/// Evaluate a gate given its input logic values.
pub fn evaluate(gate: &Gate, inputs: &[Logic]) -> Logic {
    let all_known = inputs.iter().all(|l| *l != Logic::X);
    if !all_known { return Logic::X; }
    let hi: Vec<bool> = inputs.iter().map(|l| *l == Logic::High).collect();
    let result = match gate {
        Gate::And  => hi.iter().all(|&b| b),
        Gate::Or   => hi.iter().any(|&b| b),
        Gate::Nand => !hi.iter().all(|&b| b),
        Gate::Nor  => !hi.iter().any(|&b| b),
        Gate::Xor  => hi.iter().filter(|&&b| b).count() % 2 == 1,
        Gate::Inv  => { assert_eq!(hi.len(), 1); !hi[0] },
    };
    if result { Logic::High } else { Logic::Low }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_gate() {
        assert_eq!(evaluate(&Gate::Xor, &[Logic::High, Logic::Low]), Logic::High);
        assert_eq!(evaluate(&Gate::Xor, &[Logic::High, Logic::High]), Logic::Low);
    }

    #[test]
    fn x_propagates() {
        assert_eq!(evaluate(&Gate::And, &[Logic::High, Logic::X]), Logic::X);
    }
}
