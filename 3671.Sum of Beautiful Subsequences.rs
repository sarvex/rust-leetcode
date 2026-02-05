impl Solution {
    /// Calculates total beauty of strictly increasing subsequences by GCD
    ///
    /// # Intuition
    /// Using Euler's totient identity: n = Σ_{d|n} φ(d), we get GCD(S) = Σ_{d|all elements} φ(d).
    /// Thus total_beauty = Σ_d φ(d) × f(d), where f(d) counts increasing subsequences
    /// with d dividing all elements.
    ///
    /// # Approach
    /// 1. Precompute φ(d) using sieve
    /// 2. Precompute which divisors are needed and batch-allocate BITs
    /// 3. Process elements in order; for each element v, enumerate its divisors
    /// 4. For each divisor d, use inlined Fenwick tree operations for speed
    ///
    /// # Complexity
    /// - Time: O(n × √max_val × log(max_val))
    /// - Space: O(D × max_val/D) where D = number of distinct divisors
    pub fn total_beauty(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        if nums.is_empty() {
            return 0;
        }

        let max_val = *nums.iter().max().unwrap() as usize;

        // Euler's totient using sieve
        let phi: Vec<i64> = {
            let mut phi: Vec<i64> = (0..=max_val as i64).collect();
            for i in 2..=max_val {
                if phi[i] == i as i64 {
                    let mut j = i;
                    while j <= max_val {
                        phi[j] -= phi[j] / i as i64;
                        j += i;
                    }
                }
            }
            phi
        };

        // Precompute which divisors are needed
        let needs_bit = nums
            .iter()
            .fold(vec![false; max_val + 1], |mut needs, &num| {
                let v = num as usize;
                (1..)
                    .take_while(|d| d * d <= v)
                    .filter(|d| v % d == 0)
                    .for_each(|d| {
                        needs[d] = true;
                        needs[v / d] = true;
                    });
                needs
            });

        // Batch allocate all needed BITs
        let mut bit: Vec<Vec<i64>> = (0..=max_val)
            .map(|d| {
                if d > 0 && needs_bit[d] {
                    vec![0i64; max_val / d + 2]
                } else {
                    Vec::new()
                }
            })
            .collect();

        let mut answer: i64 = 0;

        for &num in &nums {
            let v = num as usize;
            let mut d = 1;

            while d * d <= v {
                if v % d == 0 {
                    // Process small divisor d
                    let normalized = v / d;
                    {
                        let bit_d = &mut bit[d];

                        // Inline query: sum for indices < normalized
                        let mut prev = 0i64;
                        let mut idx = normalized - 1;
                        while idx > 0 {
                            prev += bit_d[idx];
                            idx -= idx & idx.wrapping_neg();
                        }

                        let cnt = (prev % MOD + 1) % MOD;
                        answer = (answer + phi[d] * cnt) % MOD;

                        // Inline update
                        let mut idx = normalized;
                        while idx < bit_d.len() {
                            bit_d[idx] = (bit_d[idx] + cnt) % MOD;
                            idx += idx & idx.wrapping_neg();
                        }
                    }

                    // Process large divisor v/d if different
                    if d * d < v {
                        let large_d = v / d;
                        let normalized = d; // v / large_d = d
                        let bit_d = &mut bit[large_d];

                        let mut prev = 0i64;
                        let mut idx = normalized - 1;
                        while idx > 0 {
                            prev += bit_d[idx];
                            idx -= idx & idx.wrapping_neg();
                        }

                        let cnt = (prev % MOD + 1) % MOD;
                        answer = (answer + phi[large_d] * cnt) % MOD;

                        let mut idx = normalized;
                        while idx < bit_d.len() {
                            bit_d[idx] = (bit_d[idx] + cnt) % MOD;
                            idx += idx & idx.wrapping_neg();
                        }
                    }
                }
                d += 1;
            }
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::total_beauty(vec![1, 2, 3]), 10);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::total_beauty(vec![4, 6]), 12);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::total_beauty(vec![5]), 5);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::total_beauty(vec![2, 2, 2]), 6);
    }

    #[test]
    fn test_descending() {
        assert_eq!(Solution::total_beauty(vec![3, 2, 1]), 6);
    }

    #[test]
    fn test_coprime_elements() {
        assert_eq!(Solution::total_beauty(vec![2, 3]), 6);
    }

    #[test]
    fn test_large_values() {
        assert_eq!(Solution::total_beauty(vec![12, 18, 24]), 78);
    }
}
