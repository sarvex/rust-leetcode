impl Solution {
    /// Counts beautiful arrangements using backtracking with precomputed divisibility.
    ///
    /// # Intuition
    /// Position i is "beautiful" if nums[i] % i == 0 or i % nums[i] == 0.
    /// Precompute valid values for each position to prune the search.
    ///
    /// # Approach
    /// 1. For each position, precompute which values satisfy the divisibility condition.
    /// 2. Backtrack through positions, placing unvisited valid values.
    /// 3. Count complete arrangements.
    ///
    /// # Complexity
    /// - Time: O(k) where k is the number of valid permutations (much less than n!)
    /// - Space: O(n²) for precomputed candidates + O(n) recursion
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let candidates: Vec<Vec<usize>> = (0..=n)
            .map(|i| {
                if i == 0 {
                    vec![]
                } else {
                    (1..=n).filter(|&j| i % j == 0 || j % i == 0).collect()
                }
            })
            .collect();

        fn backtrack(pos: usize, n: usize, candidates: &[Vec<usize>], used: &mut Vec<bool>) -> i32 {
            if pos > n {
                return 1;
            }
            let mut count = 0;
            for &j in &candidates[pos] {
                if !used[j] {
                    used[j] = true;
                    count += backtrack(pos + 1, n, candidates, used);
                    used[j] = false;
                }
            }
            count
        }

        let mut used = vec![false; n + 1];
        backtrack(1, n, &candidates, &mut used)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(Solution::count_arrangement(1), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(Solution::count_arrangement(2), 2);
    }

    #[test]
    fn test_three() {
        assert_eq!(Solution::count_arrangement(3), 3);
    }

    #[test]
    fn test_four() {
        assert_eq!(Solution::count_arrangement(4), 8);
    }

    #[test]
    fn test_five() {
        assert_eq!(Solution::count_arrangement(5), 10);
    }

    #[test]
    fn test_six() {
        assert_eq!(Solution::count_arrangement(6), 36);
    }

    #[test]
    fn test_fifteen() {
        // Known result for n=15
        assert_eq!(Solution::count_arrangement(15), 24679);
    }
}
