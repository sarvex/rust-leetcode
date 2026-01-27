impl Solution {
    /// Segment tree for counting prefix products with specific remainders.
    ///
    /// # Intuition
    /// For range [start, n-1], count how many prefixes [start, j] have product ≡ x (mod k).
    /// Each node stores cnt[r] (count of prefixes with product ≡ r) and prod (total product mod k).
    ///
    /// # Approach
    /// 1. Use fixed-size arrays since k ≤ 5 to avoid heap allocations
    /// 2. Merge: left prefixes keep counts; right prefixes get multiplied by left's product
    /// 3. Iterative update and query to reduce overhead
    ///
    /// # Complexity
    /// - Time: O((n + q) * k * log n)
    /// - Space: O(n * k)
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        // cnt[r] = count of prefixes with product ≡ r (mod k), prod = total product mod k
        // Using fixed array [i32; 5] since k ≤ 5
        let mut cnt = vec![[0i32; 5]; 4 * n];
        let mut prod = vec![1usize; 4 * n];

        #[inline]
        fn merge(cnt: &mut [[i32; 5]], prod: &mut [usize], node: usize, k: usize) {
            let left = 2 * node;
            let right = 2 * node + 1;
            cnt[node] = [0; 5];
            for r in 0..k {
                cnt[node][r] = cnt[left][r];
            }
            for r in 0..k {
                if cnt[right][r] > 0 {
                    let new_r = (prod[left] * r) % k;
                    cnt[node][new_r] += cnt[right][r];
                }
            }
            prod[node] = (prod[left] * prod[right]) % k;
        }

        fn build(
            cnt: &mut [[i32; 5]],
            prod: &mut [usize],
            nums: &[i32],
            k: usize,
            node: usize,
            start: usize,
            end: usize,
        ) {
            if start == end {
                let rem = (nums[start] % k as i32) as usize;
                cnt[node] = [0; 5];
                cnt[node][rem] = 1;
                prod[node] = rem;
                return;
            }
            let mid = (start + end) / 2;
            build(cnt, prod, nums, k, 2 * node, start, mid);
            build(cnt, prod, nums, k, 2 * node + 1, mid + 1, end);
            merge(cnt, prod, node, k);
        }

        fn update(
            cnt: &mut [[i32; 5]],
            prod: &mut [usize],
            k: usize,
            node: usize,
            start: usize,
            end: usize,
            idx: usize,
            val: i32,
        ) {
            if start == end {
                let rem = (val % k as i32) as usize;
                cnt[node] = [0; 5];
                cnt[node][rem] = 1;
                prod[node] = rem;
                return;
            }
            let mid = (start + end) / 2;
            if idx <= mid {
                update(cnt, prod, k, 2 * node, start, mid, idx, val);
            } else {
                update(cnt, prod, k, 2 * node + 1, mid + 1, end, idx, val);
            }
            merge(cnt, prod, node, k);
        }

        // Query returns (cnt_array, product)
        fn query(
            cnt: &[[i32; 5]],
            prod: &[usize],
            k: usize,
            node: usize,
            start: usize,
            end: usize,
            l: usize,
            r: usize,
        ) -> ([i32; 5], usize) {
            if r < start || end < l {
                // Empty segment: no prefixes, product identity is 1
                return ([0i32; 5], 1);
            }
            if l <= start && end <= r {
                return (cnt[node], prod[node]);
            }
            let mid = (start + end) / 2;
            let (left_cnt, left_prod) = query(cnt, prod, k, 2 * node, start, mid, l, r);
            let (right_cnt, right_prod) = query(cnt, prod, k, 2 * node + 1, mid + 1, end, l, r);

            let mut result = [0i32; 5];
            for i in 0..k {
                result[i] = left_cnt[i];
            }
            for i in 0..k {
                if right_cnt[i] > 0 {
                    let new_r = (left_prod * i) % k;
                    result[new_r] += right_cnt[i];
                }
            }
            (result, (left_prod * right_prod) % k)
        }

        build(&mut cnt, &mut prod, &nums, k, 1, 0, n - 1);

        queries
            .iter()
            .map(|q| {
                let idx = q[0] as usize;
                let val = q[1];
                let start_idx = q[2] as usize;
                let x = q[3] as usize;

                update(&mut cnt, &mut prod, k, 1, 0, n - 1, idx, val);
                let (result_cnt, _) = query(&cnt, &prod, k, 1, 0, n - 1, start_idx, n - 1);
                result_cnt[x]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 3;
        let queries = vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]];
        assert_eq!(Solution::result_array(nums, k, queries), vec![2, 2, 2]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 4, 8, 16, 32];
        let k = 4;
        let queries = vec![vec![0, 2, 0, 2], vec![0, 2, 0, 1]];
        assert_eq!(Solution::result_array(nums, k, queries), vec![1, 0]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 2;
        let queries = vec![vec![2, 1, 0, 1]];
        assert_eq!(Solution::result_array(nums, k, queries), vec![5]);
    }

    #[test]
    fn test_case_4() {
        let nums = vec![3, 9, 10, 10];
        let k = 3;
        let queries = vec![vec![1, 26, 3, 2], vec![1, 2, 2, 1], vec![1, 38, 2, 0]];
        assert_eq!(Solution::result_array(nums, k, queries), vec![0, 2, 0]);
    }
}
