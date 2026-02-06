impl Solution {
    /// Greedy with segment tree for range max queries.
    ///
    /// # Intuition
    /// The i-th '1' can move to any position in [i, pos[i]]. Processing slots from right to
    /// left, greedily pick the max available value. This works because positions that can
    /// only fill high-indexed slots get assigned first, never blocking better choices.
    ///
    /// # Approach
    /// 1. Build a segment tree storing (value, position) pairs for range max queries.
    /// 2. For each slot from k-1 to 0, query max in [i, pos[i]], add to answer, mark used.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
        let pos: Vec<usize> = s
            .bytes()
            .enumerate()
            .filter(|(_, c)| *c == b'1')
            .map(|(i, _)| i)
            .collect();
        let k = pos.len();
        if k == 0 {
            return 0;
        }
        let n = nums.len();
        let nums: Vec<i64> = nums.into_iter().map(i64::from).collect();

        let size = n.next_power_of_two();
        let mut tree = vec![(i64::MIN, 0usize); 2 * size];

        for i in 0..n {
            tree[size + i] = (nums[i], i);
        }
        for i in (1..size).rev() {
            let (left, right) = (tree[2 * i], tree[2 * i + 1]);
            tree[i] = if left.0 >= right.0 { left } else { right };
        }

        let mut ans = 0i64;
        for i in (0..k).rev() {
            let (mut l, mut r) = (size + i, size + pos[i]);
            let mut best = (i64::MIN, 0usize);
            while l <= r {
                if l & 1 == 1 {
                    if tree[l].0 > best.0 {
                        best = tree[l];
                    }
                    l += 1;
                }
                if r & 1 == 0 {
                    if tree[r].0 > best.0 {
                        best = tree[r];
                    }
                    r -= 1;
                }
                l >>= 1;
                r >>= 1;
            }
            ans += best.0;

            let mut idx = size + best.1;
            tree[idx] = (i64::MIN, best.1);
            while idx > 1 {
                idx >>= 1;
                let (left, right) = (tree[2 * idx], tree[2 * idx + 1]);
                tree[idx] = if left.0 >= right.0 { left } else { right };
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::maximum_score(vec![2, 1, 5, 2, 3], "01010".into()),
            7
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::maximum_score(vec![4, 7, 2, 9], "0000".into()), 0);
    }

    #[test]
    fn test_single_one() {
        assert_eq!(Solution::maximum_score(vec![10, 1, 1], "001".into()), 10);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::maximum_score(vec![3, 1, 2], "111".into()), 6);
    }

    #[test]
    fn test_ones_at_start() {
        assert_eq!(
            Solution::maximum_score(vec![5, 4, 3, 2, 1], "11000".into()),
            9
        );
    }

    #[test]
    fn test_order_preserved() {
        // '1's at 0,1,3: must end at (0,1,2) or (0,1,3); best is 0,1,2 -> 1+8+8=17
        assert_eq!(
            Solution::maximum_score(vec![1, 8, 8, 4, 6, 2], "110100".into()),
            17
        );
    }
}
