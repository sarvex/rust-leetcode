impl Solution {
    /// Number of Distinct Roll Sequences
    ///
    /// # Intuition
    /// Track the last two dice rolls to enforce both constraints: GCD of adjacent
    /// rolls must be 1, and equal values must have a gap greater than 2.
    ///
    /// # Approach
    /// Use dynamic programming with state (prev_roll, curr_roll). Precompute which
    /// dice pairs are coprime (GCD = 1). For each new roll, check:
    /// 1. GCD(new_roll, curr_roll) == 1
    /// 2. new_roll != curr_roll (implied by GCD check except for 1)
    /// 3. new_roll != prev_roll
    ///
    /// # Complexity
    /// - Time: O(n * 6³) = O(n)
    /// - Space: O(6²) = O(1)
    pub fn distinct_sequences(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Precomputed list of valid next rolls for each current roll (1-indexed internally as 0-5)
        // valid_next[i] contains all j where gcd(i+1, j+1) == 1 and i != j
        const VALID_NEXT: [&[usize]; 6] = [
            &[1, 2, 3, 4, 5], // From 1: can go to 2,3,4,5,6
            &[0, 2, 4],       // From 2: can go to 1,3,5
            &[0, 1, 3, 4],    // From 3: can go to 1,2,4,5
            &[0, 2, 4],       // From 4: can go to 1,3,5
            &[0, 1, 2, 3, 5], // From 5: can go to 1,2,3,4,6
            &[0, 4],          // From 6: can go to 1,5
        ];

        let n = n as usize;

        if n == 1 {
            return 6;
        }

        // dp[prev][curr] = count of sequences ending with (prev+1, curr+1)
        // Use 6 as sentinel for "no previous roll" in first transition
        let mut dp = [[0i64; 6]; 7];

        // Initialize for n=1: any single roll, prev = sentinel (6)
        for curr in 0..6 {
            dp[6][curr] = 1;
        }

        // Build sequences of length 2
        let mut new_dp = [[0i64; 6]; 7];
        for curr in 0..6 {
            for &next in VALID_NEXT[curr] {
                new_dp[curr][next] = (new_dp[curr][next] + dp[6][curr]) % MOD;
            }
        }
        dp = new_dp;

        // Build sequences of length 3 to n
        for _ in 3..=n {
            new_dp = [[0i64; 6]; 7];
            for prev in 0..6 {
                for curr in 0..6 {
                    if dp[prev][curr] == 0 {
                        continue;
                    }
                    for &next in VALID_NEXT[curr] {
                        // Ensure gap > 2 between equal values
                        if next != prev {
                            new_dp[curr][next] = (new_dp[curr][next] + dp[prev][curr]) % MOD;
                        }
                    }
                }
            }
            dp = new_dp;
        }

        // Sum all valid ending states
        let mut result = 0i64;
        for prev in 0..7 {
            for curr in 0..6 {
                result = (result + dp[prev][curr]) % MOD;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_equals_1() {
        assert_eq!(Solution::distinct_sequences(1), 6);
    }

    #[test]
    fn test_n_equals_2() {
        assert_eq!(Solution::distinct_sequences(2), 22);
    }

    #[test]
    fn test_n_equals_4() {
        assert_eq!(Solution::distinct_sequences(4), 184);
    }

    #[test]
    fn test_n_equals_3() {
        // Manual verification: valid sequences of length 3
        assert_eq!(Solution::distinct_sequences(3), 66);
    }

    #[test]
    fn test_large_n() {
        // Verify it handles large inputs without overflow
        let result = Solution::distinct_sequences(10000);
        assert!(result >= 0);
    }
}
