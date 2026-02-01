const MOD: i64 = 1_000_000_007;
const NEG_INF: i64 = -4_000_000_000_000_000_000;

/// Node value: dp[first_taken][last_taken] for this segment.
/// Use NEG_INF for invalid configurations.
#[derive(Clone, Copy)]
struct Node {
    dp: [[i64; 2]; 2],
}

impl Node {
    fn leaf(value: i64) -> Self {
        let mut dp = [[NEG_INF; 2]; 2];
        dp[0][0] = 0;
        dp[1][1] = value;
        Self { dp }
    }

    fn merge(left: Self, right: Self) -> Self {
        let mut dp = [[NEG_INF; 2]; 2];
        for first_taken in 0..=1 {
            for last_taken in 0..=1 {
                let mut best = NEG_INF;
                for left_last in 0..=1 {
                    for right_first in 0..=1 {
                        if left_last == 1 && right_first == 1 {
                            continue;
                        }
                        let left_val = left.dp[first_taken][left_last];
                        let right_val = right.dp[right_first][last_taken];
                        if left_val == NEG_INF || right_val == NEG_INF {
                            continue;
                        }
                        best = best.max(left_val + right_val);
                    }
                }
                dp[first_taken][last_taken] = best;
            }
        }
        Self { dp }
    }

    fn best(self) -> i64 {
        let mut best = 0;
        for first_taken in 0..=1 {
            for last_taken in 0..=1 {
                best = best.max(self.dp[first_taken][last_taken]);
            }
        }
        best
    }
}

struct SegTree {
    levels: Vec<Vec<Node>>,
}

impl SegTree {
    fn new(nums: &[i32]) -> Self {
        let mut first_level = Vec::with_capacity(nums.len());
        for &num in nums {
            first_level.push(Node::leaf(num as i64));
        }

        let mut levels = Vec::new();
        levels.push(first_level);

        while levels.last().unwrap().len() > 1 {
            let prev = levels.last().unwrap();
            let mut next = Vec::with_capacity((prev.len() + 1) / 2);
            let mut idx = 0usize;
            while idx < prev.len() {
                if idx + 1 < prev.len() {
                    next.push(Node::merge(prev[idx], prev[idx + 1]));
                } else {
                    next.push(prev[idx]);
                }
                idx += 2;
            }
            levels.push(next);
        }

        Self { levels }
    }

    fn update(&mut self, pos: usize, value: i32) {
        self.levels[0][pos] = Node::leaf(value as i64);
        let mut idx = pos / 2;
        for level in 1..self.levels.len() {
            let left_idx = idx * 2;
            let left = self.levels[level - 1][left_idx];
            if left_idx + 1 < self.levels[level - 1].len() {
                let right = self.levels[level - 1][left_idx + 1];
                self.levels[level][idx] = Node::merge(left, right);
            } else {
                self.levels[level][idx] = left;
            }
            idx /= 2;
        }
    }

    fn root_best(&self) -> i64 {
        self.levels.last().unwrap()[0].best()
    }
}

impl Solution {
    /// Segment tree over "max non-adjacent subsequence sum" with point updates.
    ///
    /// # Intuition
    /// Max sum of a subsequence with no two adjacent elements is a classic DP; supporting
    /// point updates and full-array query in O(log n) per operation requires a segment tree.
    /// Each node stores dp[first_taken][last_taken]; merging forbids taking both L's last and
    /// R's first.
    ///
    /// # Approach
    /// 1. Build a segment tree where each node holds dp[first_taken][last_taken] for its segment.
    /// 2. For each query: update nums[pos] = x, then take the root's best as the answer.
    /// 3. Sum all query answers and return modulo 10^9+7.
    ///
    /// # Complexity
    /// - Time: O((n + q) log n)
    /// - Space: O(n)
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut tree = SegTree::new(&nums);
        let mut total: i64 = 0;
        for query in &queries {
            if query.len() != 2 {
                continue;
            }
            let pos = query[0] as usize;
            let new_value = query[1];
            tree.update(pos, new_value);
            let best = tree.root_best();
            total = (total + best).rem_euclid(MOD);
        }
        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![3, 5, 9];
        let queries = vec![vec![1, -2], vec![0, -3]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 21);
    }

    #[test]
    fn test_example2() {
        let nums = vec![0, -1];
        let queries = vec![vec![0, -5]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 0);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![10];
        let queries = vec![vec![0, 10]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 10);
    }

    #[test]
    fn test_all_negative() {
        let nums = vec![-1, -2, -3];
        let queries = vec![vec![0, -1], vec![1, -2], vec![2, -3]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 0);
    }

    #[test]
    fn test_nums_03331_neg2_queries_4_0_and_1_0() {
        let nums = vec![0, 3, 3, 3, 1, -2];
        let queries = vec![vec![4, 0], vec![1, 0]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 9);
    }

    #[test]
    fn test_nums_4_0_neg1_neg2_3_1_neg1_queries_8() {
        let nums = vec![4, 0, -1, -2, 3, 1, -1];
        let queries = vec![
            vec![3, 1],
            vec![0, -2],
            vec![1, -1],
            vec![0, -2],
            vec![5, 4],
            vec![6, -3],
            vec![6, -2],
            vec![2, -1],
        ];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 36);
    }
}
