impl Solution {
    /// Maximize Alternating Sum Using Swaps via Union-Find
    ///
    /// # Intuition
    /// Swaps define an equivalence relation - indices connected through swaps can freely
    /// exchange values. To maximize alternating sum, we want larger values at even indices
    /// (positive contribution) and smaller values at odd indices (negative contribution).
    ///
    /// # Approach
    /// 1. Use Union-Find with iterative path compression to group indices
    /// 2. For each connected component, count even and odd positions
    /// 3. Sort values descending, assign largest to even positions, smallest to odd
    /// 4. Sum contributions across all components
    ///
    /// # Complexity
    /// - Time: O(n α(n) + n log n) where α is inverse Ackermann
    /// - Space: O(n)
    pub fn max_alternating_sum(nums: Vec<i32>, swaps: Vec<Vec<i32>>) -> i64 {
        let n = nums.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<u8> = vec![0; n];

        #[inline]
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            let mut root = x;
            while parent[root] != root {
                root = parent[root];
            }
            while parent[x] != root {
                let next = parent[x];
                parent[x] = root;
                x = next;
            }
            root
        }

        for swap in &swaps {
            let (mut px, mut py) = (find(&mut parent, swap[0] as usize), find(&mut parent, swap[1] as usize));
            if px != py {
                if rank[px] < rank[py] {
                    std::mem::swap(&mut px, &mut py);
                }
                parent[py] = px;
                if rank[px] == rank[py] {
                    rank[px] += 1;
                }
            }
        }

        let mut groups: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..n {
            groups[find(&mut parent, i)].push(i);
        }

        groups
            .into_iter()
            .filter(|g| !g.is_empty())
            .map(|indices| {
                let mut values: Vec<i64> = indices.iter().map(|&i| nums[i] as i64).collect();
                let even_count = indices.iter().filter(|&&i| i % 2 == 0).count();

                values.sort_unstable_by(|a, b| b.cmp(a));

                values[..even_count].iter().sum::<i64>() - values[even_count..].iter().sum::<i64>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        swaps: Vec<Vec<i32>>,
        expected: i64,
    }

    #[test]
    fn test_max_alternating_sum() {
        let test_cases = vec![
            TestCase {
                nums: vec![1, 2, 3],
                swaps: vec![vec![0, 2], vec![1, 2]],
                expected: 4,
            },
            TestCase {
                nums: vec![1, 2, 3],
                swaps: vec![vec![1, 2]],
                expected: 2,
            },
            TestCase {
                nums: vec![1, 1000000000, 1, 1000000000, 1, 1000000000],
                swaps: vec![],
                expected: -2999999997,
            },
            TestCase {
                nums: vec![5, 3],
                swaps: vec![vec![0, 1]],
                expected: 2,
            },
            TestCase {
                nums: vec![10, 20, 30, 40],
                swaps: vec![vec![0, 1], vec![2, 3]],
                expected: 20,
            },
        ];

        for (i, tc) in test_cases.iter().enumerate() {
            let result = Solution::max_alternating_sum(tc.nums.clone(), tc.swaps.clone());
            assert_eq!(result, tc.expected, "Test case {} failed", i + 1);
        }
    }

    #[test]
    fn test_all_connected() {
        let nums = vec![1, 5, 2, 4, 3];
        let swaps = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = Solution::max_alternating_sum(nums, swaps);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_no_swaps() {
        let nums = vec![10, 5, 8, 3];
        let swaps = vec![];
        let result = Solution::max_alternating_sum(nums, swaps);
        assert_eq!(result, 10 - 5 + 8 - 3);
    }
}
