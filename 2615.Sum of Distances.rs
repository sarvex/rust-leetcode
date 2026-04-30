use std::collections::HashMap;

impl Solution {
    /// Group indices by value, then apply an incremental prefix formula per group.
    ///
    /// # Intuition
    /// For a single value, let its occurrence indices be `p_0 < p_1 < ... < p_{k-1}`.
    /// The answer at `p_0` is `sum(p_j) - k * p_0` (every other occurrence lies to
    /// the right). Moving from `p_{i-1}` to `p_i`, the `i` left neighbours each move
    /// `(p_i - p_{i-1})` closer-from-the-left (contribute `+`) while the `k - i`
    /// right neighbours each move that much further-from-the-right (contribute `-`),
    /// yielding the recurrence `arr[p_i] = arr[p_{i-1}] + (2i - k) * (p_i - p_{i-1})`.
    /// This turns a naive O(n^2) per-pair computation into a single linear sweep.
    ///
    /// # Approach
    /// 1. Bucket indices by their value using a `HashMap<i32, Vec<usize>>`.
    /// 2. For each bucket of size `k`:
    ///    - Compute `total = sum of indices`.
    ///    - Seed `arr[p_0] = total - k * p_0`.
    ///    - Sweep left-to-right applying the delta recurrence above.
    /// 3. Singleton buckets keep the default value `0`.
    ///
    /// All arithmetic is performed in `i64` because `n * n` can reach `10^10`,
    /// which overflows `i32`.
    ///
    /// # Complexity
    /// - Time: `O(n)` — each index is visited exactly twice (bucket build + sweep).
    /// - Space: `O(n)` — buckets plus the output vector.
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::with_capacity(n);
        for (i, &v) in nums.iter().enumerate() {
            groups.entry(v).or_default().push(i);
        }

        let mut arr = vec![0i64; n];
        for indices in groups.values() {
            let k = indices.len();
            if k < 2 {
                continue;
            }
            let total: i64 = indices.iter().map(|&p| p as i64).sum();
            let k_i64 = k as i64;

            let first = indices[0] as i64;
            arr[indices[0]] = total - k_i64 * first;

            for i in 1..k {
                let prev = indices[i - 1] as i64;
                let curr = indices[i] as i64;
                let delta = (2 * i as i64 - k_i64) * (curr - prev);
                arr[indices[i]] = arr[indices[i - 1]] + delta;
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::distance(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
    }

    #[test]
    fn test_example_2_all_distinct() {
        assert_eq!(Solution::distance(vec![0, 5, 3]), vec![0, 0, 0]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::distance(vec![42]), vec![0]);
    }

    #[test]
    fn test_all_same() {
        // indices [0,1,2,3]; for i=0: 1+2+3=6, i=1: 1+1+2=4,
        // i=2: 2+1+1=4, i=3: 3+2+1=6.
        assert_eq!(Solution::distance(vec![7, 7, 7, 7]), vec![6, 4, 4, 6]);
    }

    #[test]
    fn test_two_same_elements() {
        // Pairs at indices 0 and 3: arr[0] = 3, arr[3] = 3, others 0.
        assert_eq!(Solution::distance(vec![5, 1, 2, 5, 9]), vec![3, 0, 0, 3, 0]);
    }

    #[test]
    fn test_large_values() {
        // Values up to 10^9 must work because grouping key is i32.
        let nums = vec![1_000_000_000, 0, 1_000_000_000, 0];
        // indices for 1e9: [0,2] -> arr[0]=2, arr[2]=2
        // indices for 0:   [1,3] -> arr[1]=2, arr[3]=2
        assert_eq!(Solution::distance(nums), vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_stress_large_group_no_overflow() {
        // 100_000 identical values; verifies i64 safety and O(n) performance.
        let n = 100_000usize;
        let nums = vec![1; n];
        let result = Solution::distance(nums);

        // Validate the endpoints against the closed form:
        //   arr[0] = sum_{j=1..n-1} j = n*(n-1)/2
        let expected_first = (n as i64) * (n as i64 - 1) / 2;
        assert_eq!(result[0], expected_first);
        assert_eq!(result[n - 1], expected_first);

        // Middle index m has arr[m] = m*(m+1)/2 + (n-1-m)*(n-m)/2
        let m = n / 2;
        let left = (m as i64) * (m as i64 + 1) / 2;
        let right = (n as i64 - 1 - m as i64) * (n as i64 - m as i64) / 2;
        assert_eq!(result[m], left + right);
    }

    #[test]
    fn test_mixed_groups() {
        // nums = [2,2,2,3,3]
        // Group 2 at [0,1,2]: arr[0]=1+2=3, arr[1]=1+1=2, arr[2]=2+1=3
        // Group 3 at [3,4]:   arr[3]=1, arr[4]=1
        assert_eq!(Solution::distance(vec![2, 2, 2, 3, 3]), vec![3, 2, 3, 1, 1]);
    }
}
