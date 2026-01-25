impl Solution {
    /// Finds the smallest special palindrome number strictly greater than n.
    ///
    /// # Intuition
    /// A special number is a palindrome where each digit k appears exactly k times.
    /// We can use at most one odd digit combined with any subset of even digits {2,4,6,8}.
    ///
    /// # Approach
    /// 1. Skip lengths smaller than n's digit count (can't be > n)
    /// 2. For lengths > n's digit count, only compute the smallest palindrome per combination
    /// 3. For same length, generate candidates and filter by > n
    /// 4. Track the global minimum to enable early termination
    ///
    /// # Complexity
    /// - Time: O(C * P) where C is valid combinations, P is permutations per combination
    /// - Space: O(L) where L is max palindrome length
    pub fn special_palindrome(n: i64) -> i64 {
        let n_len = Self::digit_count(n);
        let mut best = i64::MAX;

        for odd in [0usize, 1, 3, 5, 7, 9] {
            for mask in 0u32..16 {
                let mut counts = [0usize; 10];
                let mut total = 0;

                if odd > 0 {
                    counts[odd] = odd;
                    total += odd;
                }

                for (i, &d) in [2usize, 4, 6, 8].iter().enumerate() {
                    if mask & (1 << i) != 0 {
                        counts[d] = d;
                        total += d;
                    }
                }

                if total == 0 || total > 18 || total < n_len {
                    continue;
                }

                if total > n_len {
                    let candidate = Self::smallest_palindrome(&counts, total);
                    best = best.min(candidate);
                } else {
                    Self::find_smallest_greater(&counts, total, n, &mut best);
                }
            }
        }

        best
    }

    fn digit_count(n: i64) -> usize {
        if n == 0 {
            return 1;
        }
        ((n as f64).log10().floor() as usize) + 1
    }

    fn smallest_palindrome(counts: &[usize; 10], total: usize) -> i64 {
        let half_len = total / 2;
        let is_odd = total % 2 == 1;

        let mut first_half = Vec::with_capacity(half_len);
        let mut middle = 0u8;

        for d in 1..=9 {
            for _ in 0..counts[d] / 2 {
                first_half.push(d as u8);
            }
            if counts[d] % 2 == 1 {
                middle = d as u8;
            }
        }

        Self::build_palindrome(&first_half, is_odd, middle)
    }

    fn find_smallest_greater(counts: &[usize; 10], total: usize, n: i64, best: &mut i64) {
        let half_len = total / 2;
        let is_odd = total % 2 == 1;

        let mut half_counts = [0usize; 10];
        let mut middle = 0u8;

        for d in 1..=9 {
            half_counts[d] = counts[d] / 2;
            if counts[d] % 2 == 1 {
                middle = d as u8;
            }
        }

        let mut current = Vec::with_capacity(half_len);
        Self::generate(&mut half_counts, &mut current, half_len, is_odd, middle, n, best);
    }

    fn generate(
        half_counts: &mut [usize; 10],
        current: &mut Vec<u8>,
        half_len: usize,
        is_odd: bool,
        middle: u8,
        n: i64,
        best: &mut i64,
    ) {
        if current.len() == half_len {
            let num = Self::build_palindrome(current, is_odd, middle);
            if num > n && num < *best {
                *best = num;
            }
            return;
        }

        for d in 1..=9 {
            if half_counts[d] > 0 {
                half_counts[d] -= 1;
                current.push(d as u8);
                Self::generate(half_counts, current, half_len, is_odd, middle, n, best);
                current.pop();
                half_counts[d] += 1;
            }
        }
    }

    fn build_palindrome(first_half: &[u8], is_odd: bool, middle: u8) -> i64 {
        let mut num: i64 = 0;

        for &d in first_half {
            num = num * 10 + d as i64;
        }

        if is_odd {
            num = num * 10 + middle as i64;
        }

        for &d in first_half.iter().rev() {
            num = num * 10 + d as i64;
        }

        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::special_palindrome(2), 22);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::special_palindrome(33), 212);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::special_palindrome(0), 1);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::special_palindrome(1), 22);
    }

    #[test]
    fn test_after_212() {
        assert_eq!(Solution::special_palindrome(212), 333);
    }

    #[test]
    fn test_large() {
        assert_eq!(Solution::special_palindrome(1000000), 2441442);
    }

    #[test]
    fn test_very_large() {
        let result = Solution::special_palindrome(999_999_999_999_999);
        assert!(result > 999_999_999_999_999);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(Solution::special_palindrome(333), 4444);
        assert_eq!(Solution::special_palindrome(4444), 23332);
    }
}
