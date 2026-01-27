impl Solution {
    /// Factorial number system decomposition for kth permutation sequence.
    ///
    /// # Intuition
    /// The kth permutation can be determined digit by digit using factorial
    /// base. At each position, the number of permutations with a fixed first
    /// digit is `(n-1)!`. Dividing k by this count identifies which digit
    /// to place next.
    ///
    /// # Approach
    /// Precompute factorials. For each position, compute how many complete
    /// groups of permutations to skip (`k / factorial`), select the
    /// corresponding unused digit, mark it used, and reduce k by the
    /// skipped count.
    ///
    /// # Complexity
    /// - Time: O(n^2) — n positions × linear scan for unused digits
    /// - Space: O(n) — factorial array and used-digit tracker
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize - 1;
        let mut factorials = vec![1usize; n];
        for i in 1..n {
            factorials[i] = factorials[i - 1] * i;
        }

        let mut available: Vec<u8> = (1..=n as u8).collect();
        let mut result = String::with_capacity(n);

        for i in (0..n).rev() {
            let index = k / factorials[i];
            k %= factorials[i];
            result.push((available.remove(index) + b'0') as char);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n3_k3() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
    }

    #[test]
    fn n4_k9() {
        assert_eq!(Solution::get_permutation(4, 9), "2314");
    }

    #[test]
    fn n3_k1() {
        assert_eq!(Solution::get_permutation(3, 1), "123");
    }

    #[test]
    fn last_permutation() {
        assert_eq!(Solution::get_permutation(3, 6), "321");
    }
}
