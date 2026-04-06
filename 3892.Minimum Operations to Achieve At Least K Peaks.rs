impl Solution {
    /// Minimum operations for at least k peaks via circular non-adjacent selection DP.
    ///
    /// # Intuition
    /// Peaks must be non-adjacent in the circular array. Since only increments are
    /// allowed and peaks are non-adjacent, the cost to create each peak is independent:
    /// `max(0, max(left_neighbor, right_neighbor) + 1 - nums[i])`. The problem reduces
    /// to selecting k non-adjacent elements in a circular cost array with minimum sum.
    ///
    /// # Approach
    /// 1. Compute per-index peak cost from original neighbor values.
    /// 2. Split the circular non-adjacency constraint into two linear cases:
    ///    - Index 0 excluded: pick k non-adjacent from indices `1..n`.
    ///    - Index 0 included: pick k−1 non-adjacent from indices `2..n−1`.
    /// 3. Solve each linear case with rolling-array DP in O(m·k) time.
    ///
    /// # Complexity
    /// - Time: O(n·k)
    /// - Space: O(k)
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        if k == 0 {
            return 0;
        }
        if k > n / 2 {
            return -1;
        }

        let cost: Vec<i64> = (0..n)
            .map(|i| {
                let left = nums[(i + n - 1) % n] as i64;
                let right = nums[(i + 1) % n] as i64;
                (left.max(right) + 1 - nums[i] as i64).max(0)
            })
            .collect();

        let case1 = Self::pick_min(&cost[1..], k);
        let case2 = if k == 1 {
            cost[0]
        } else if n >= 4 {
            match Self::pick_min(&cost[2..n - 1], k - 1) {
                i64::MAX => i64::MAX,
                v => cost[0] + v,
            }
        } else {
            i64::MAX
        };

        match case1.min(case2) {
            i64::MAX => -1,
            v => v as i32,
        }
    }

    /// Minimum cost of selecting exactly `want` non-adjacent elements from a linear array.
    fn pick_min(arr: &[i64], want: usize) -> i64 {
        let m = arr.len();
        if want == 0 {
            return 0;
        }
        if want > (m + 1) / 2 {
            return i64::MAX;
        }

        let mut pp = vec![i64::MAX; want + 1];
        pp[0] = 0;
        let mut p = vec![i64::MAX; want + 1];
        p[0] = 0;
        p[1] = arr[0];
        let mut cur = vec![0i64; want + 1];

        for i in 2..=m {
            for j in 0..=want {
                cur[j] = p[j];
                if j > 0 && pp[j - 1] != i64::MAX {
                    cur[j] = cur[j].min(pp[j - 1] + arr[i - 1]);
                }
            }
            std::mem::swap(&mut pp, &mut p);
            std::mem::swap(&mut p, &mut cur);
        }

        p[want]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::min_operations(vec![2, 1, 2], 1), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_operations(vec![4, 5, 3, 6], 2), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::min_operations(vec![3, 7, 3], 2), -1);
    }

    #[test]
    fn zero_peaks() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], 0), 0);
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::min_operations(vec![5, 3], 1), 0);
        assert_eq!(Solution::min_operations(vec![5, 5], 1), 1);
    }

    #[test]
    fn all_equal() {
        assert_eq!(Solution::min_operations(vec![5, 5, 5, 5], 2), 2);
    }

    #[test]
    fn impossible() {
        assert_eq!(Solution::min_operations(vec![1, 2], 2), -1);
    }
}
