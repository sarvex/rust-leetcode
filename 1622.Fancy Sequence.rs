const MOD: i64 = 1_000_000_007;

/// Fancy sequence with lazy propagation of add/mult operations.
///
/// # Intuition
/// Instead of applying addAll/multAll to each element individually (O(n) per operation),
/// we track cumulative operations and apply them lazily only when needed (O(1) per getIndex).
/// Each element stores its "original" value before any operations were applied.
///
/// # Approach
/// - Store elements in their original form: actual_value = stored * mult + add
/// - For append: store the value in "base" form by reversing current operations
/// - For addAll: update add += inc
/// - For multAll: update add *= m, mult *= m (order matters!)
/// - For getIndex: compute stored * mult + add
///
/// # Complexity
/// - append: O(1)
/// - addAll: O(1)
/// - multAll: O(1)
/// - getIndex: O(1)
/// - Space: O(n)
struct Fancy {
    data: Vec<i64>, // stored original values
    add: i64,       // cumulative addition
    mult: i64,      // cumulative multiplication
}

impl Fancy {
    fn new() -> Self {
        Fancy {
            data: Vec::new(),
            add: 0,
            mult: 1,
        }
    }

    /// Modular exponentiation: base^exp mod MOD
    fn mod_pow(base: i64, exp: i64) -> i64 {
        fn mod_mul(a: i64, b: i64) -> i64 {
            ((a as u128 * b as u128) % (MOD as u128)) as i64
        }

        let mut result: i64 = 1;
        let mut b = (base % MOD + MOD) % MOD;
        let mut e = exp;

        while e > 0 {
            if e & 1 == 1 {
                result = mod_mul(result, b);
            }
            b = mod_mul(b, b);
            e >>= 1;
        }
        result
    }

    /// Modular inverse using Fermat's little theorem: x^(-1) ≡ x^(MOD-2) mod MOD
    fn mod_inv(x: i64) -> i64 {
        Self::mod_pow(x, MOD - 2)
    }

    /// Appends an integer val to the end of the sequence.
    ///
    /// To store the value correctly for lazy operations, we reverse-transform:
    /// stored = (val - add) * inv(mult) mod MOD
    /// This ensures getIndex returns: stored * mult + add = val
    fn append(&mut self, val: i32) {
        let val = val as i64;
        // Reverse the cumulative operations to store original value
        let adjusted = ((val - self.add) % MOD + MOD) % MOD;
        let stored = ((adjusted as u128 * Self::mod_inv(self.mult) as u128) % (MOD as u128)) as i64;
        self.data.push(stored);
    }

    /// Increments all existing values in the sequence by inc.
    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }

    /// Multiplies all existing values in the sequence by m.
    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.add = ((self.add as u128 * m as u128) % (MOD as u128)) as i64;
        self.mult = ((self.mult as u128 * m as u128) % (MOD as u128)) as i64;
    }

    /// Gets the current value at index idx. Returns -1 if idx is out of bounds.
    fn get_index(&self, idx: i32) -> i32 {
        if idx < 0 || idx as usize >= self.data.len() {
            return -1;
        }
        let val = ((self.data[idx as usize] as u128 * self.mult as u128 + self.add as u128)
            % (MOD as u128)) as i64;
        val as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut fancy = Fancy::new();

        fancy.append(2); // [2]
        fancy.add_all(3); // [5]
        fancy.append(7); // [5, 7]
        fancy.mult_all(2); // [10, 14]
        assert_eq!(fancy.get_index(0), 10);

        fancy.add_all(3); // [13, 17]
        fancy.append(10); // [13, 17, 10]
        fancy.mult_all(2); // [26, 34, 20]
        assert_eq!(fancy.get_index(0), 26);
        assert_eq!(fancy.get_index(1), 34);
        assert_eq!(fancy.get_index(2), 20);
    }

    #[test]
    fn test_empty_sequence() {
        let fancy = Fancy::new();
        assert_eq!(fancy.get_index(0), -1);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut fancy = Fancy::new();
        fancy.append(5);
        assert_eq!(fancy.get_index(0), 5);
        assert_eq!(fancy.get_index(1), -1);
        assert_eq!(fancy.get_index(100), -1);
    }

    #[test]
    fn test_multiple_mult_all() {
        let mut fancy = Fancy::new();
        fancy.append(2);
        fancy.add_all(1); // [3]
        fancy.mult_all(2); // [6]
        fancy.add_all(2); // [8]
        fancy.mult_all(3); // [24]
        assert_eq!(fancy.get_index(0), 24);
    }

    #[test]
    fn test_append_after_operations() {
        let mut fancy = Fancy::new();
        fancy.append(2);
        fancy.add_all(3); // [5]
        fancy.append(10); // should be [5, 10], not [5, 13]
        assert_eq!(fancy.get_index(0), 5);
        assert_eq!(fancy.get_index(1), 10);
    }

    #[test]
    fn test_large_values() {
        let mut fancy = Fancy::new();
        fancy.append(100);
        fancy.add_all(100);
        fancy.mult_all(100);
        // (100 + 100) * 100 = 20000
        assert_eq!(fancy.get_index(0), 20000);
    }

    #[test]
    fn test_modulo() {
        let mut fancy = Fancy::new();
        fancy.append(1_000_000_000);
        fancy.add_all(1_000_000_000);
        fancy.mult_all(2);
        // (1e9 + 1e9) * 2 = 4e9 % MOD = 4e9 - MOD = 4e9 - 1000000007 = 299999993
        let expected = ((1_000_000_000 + 1_000_000_000) * 2) % MOD;
        assert_eq!(fancy.get_index(0), expected as i32);
    }
}
