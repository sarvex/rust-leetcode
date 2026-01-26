impl Solution {
    /// Counts connected components where nodes connect if their LCM ≤ threshold.
    ///
    /// # Intuition
    /// Two numbers a and b are connected if lcm(a, b) ≤ threshold. Key insight:
    /// if both a and b divide some m ≤ threshold, then lcm(a, b) ≤ m ≤ threshold.
    ///
    /// # Approach
    /// 1. Numbers > threshold are isolated (lcm with anything exceeds threshold)
    /// 2. For numbers ≤ threshold, use Union-Find: iterate through multiples m
    ///    up to threshold and union all numbers that share a common multiple
    ///
    /// # Complexity
    /// - Time: O(threshold × log(threshold) × α(n)) where α is inverse Ackermann
    /// - Space: O(threshold)
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let threshold = threshold as usize;

        let (isolated, valid): (Vec<_>, Vec<_>) = nums
            .into_iter()
            .map(|n| n as usize)
            .partition(|&n| n > threshold);

        if valid.is_empty() {
            return isolated.len() as i32;
        }

        let mut parent: Vec<usize> = (0..=threshold).collect();
        let mut first = vec![0usize; threshold + 1];

        for &v in &valid {
            let mut m = v;
            while m <= threshold {
                if first[m] == 0 {
                    first[m] = v;
                } else {
                    let px = Self::find(&mut parent, first[m]);
                    let py = Self::find(&mut parent, v);
                    if px != py {
                        parent[px] = py;
                    }
                }
                m += v;
            }
        }

        let roots: std::collections::HashSet<_> =
            valid.iter().map(|&v| Self::find(&mut parent, v)).collect();

        isolated.len() as i32 + roots.len() as i32
    }

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 4, 8, 3, 9];
        let threshold = 5;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 4, 8, 3, 9, 12];
        let threshold = 10;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_all_isolated() {
        let nums = vec![100, 200, 300];
        let threshold = 5;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        let threshold = 10;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_all_connected_via_one() {
        let nums = vec![1, 2, 3];
        let threshold = 6;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_lcm_equals_threshold() {
        let nums = vec![2, 3];
        let threshold = 6;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_lcm_exceeds_threshold() {
        let nums = vec![2, 3];
        let threshold = 5;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_divisibility_chain() {
        let nums = vec![2, 4, 8];
        let threshold = 8;

        let result = Solution::count_components(nums, threshold);

        assert_eq!(result, 1);
    }
}
