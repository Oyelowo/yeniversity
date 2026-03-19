//! Digital logic: boolean evaluation, truth tables, half/full adder.

/// Half adder: returns (sum, carry)
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (a ^ b, a & b)
}

/// Full adder: returns (sum, carry_out)
pub fn full_adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (s1, c1) = half_adder(a, b);
    let (sum, c2) = half_adder(s1, carry_in);
    (sum, c1 | c2)
}

/// N-bit ripple-carry adder. Returns result bits (LSB first) and final carry-out.
pub fn ripple_carry_add(a: &[bool], b: &[bool]) -> (Vec<bool>, bool) {
    assert_eq!(a.len(), b.len());
    let mut carry = false;
    let mut result = Vec::with_capacity(a.len());
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        let (s, c) = full_adder(ai, bi, carry);
        result.push(s);
        carry = c;
    }
    (result, carry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_adder_1_plus_1_plus_1() {
        let (sum, carry) = full_adder(true, true, true);
        assert!(sum);   // 1+1+1 = 3 = 0b11: sum bit = 1
        assert!(carry); // carry = 1
    }

    #[test]
    fn ripple_4bit_addition() {
        // 0b0110 (6) + 0b0101 (5) = 0b1011 (11), no overflow for 4-bit
        let a = [false, true, true, false]; // 6 in LSB-first
        let b = [true, false, true, false]; // 5 in LSB-first
        let (result, overflow) = ripple_carry_add(&a, &b);
        // 11 = 0b1011 → LSB-first: [1,1,0,1]
        assert_eq!(result, [true, true, false, true]);
        assert!(!overflow);
    }
}
