impl Solution {
    /// Count valid substrings per query with prefix sums and a monotone boundary.
    ///
    /// # Intuition
    /// A substring is invalid only when it contains more than `k` zeros and more than `k` ones.
    /// For each right endpoint, there is a minimal left index that makes the substring valid, and
    /// all starts to the right are valid as well.
    ///
    /// # Approach
    /// - Sweep with a sliding window to maintain the minimal valid left for each right endpoint.
    ///   The number of valid substrings ending at `right` is `right - left + 1`.
    /// - Build a prefix sum array over these counts.
    /// - For a query `[l, r]`, take `prefix[r] - prefix[l - 1]` and correct the overcount for
    ///   rights whose minimal valid left is `< l` using a binary search and an arithmetic series.
    ///
    /// # Complexity
    /// - Time: O(n + q log n)
    /// - Space: O(n)
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 {
            return vec![0; queries.len()];
        }
        let mut prefix_valid = vec![0_i64; n];

        let (mut left, mut count_zero, mut count_one) = (0_usize, 0_i32, 0_i32);
        for right in 0..n {
            if bytes[right] == b'1' {
                count_one += 1;
            } else {
                count_zero += 1;
            }

            while count_one > k && count_zero > k {
                if bytes[left] == b'1' {
                    count_one -= 1;
                } else {
                    count_zero -= 1;
                }
                left += 1;
            }

            let valid_ending = (right - left + 1) as i64;
            prefix_valid[right] = valid_ending
                + if right > 0 {
                    prefix_valid[right - 1]
                } else {
                    0
                };
        }

        let mut answers = vec![0_i64; queries.len()];
        for (idx, query) in queries.iter().enumerate() {
            let query_left = query[0] as usize;
            let query_right = query[1] as usize;

            if query_left == 0 {
                answers[idx] = prefix_valid[query_right];
                continue;
            }

            let mut total = prefix_valid[query_right] - prefix_valid[query_left - 1];
            let mut low = query_left;
            let mut high = query_right;

            while low < high {
                let mid = (low + high + 1) / 2;
                let valid_ending = prefix_valid[mid] - prefix_valid[mid - 1];
                if valid_ending < (mid - query_left + 1) as i64 {
                    high = mid - 1;
                } else {
                    low = mid;
                }
            }

            let boundary = low;
            let current = prefix_valid[boundary] - prefix_valid[query_left - 1];
            let len = (boundary - query_left + 1) as i64;
            let desired = len * (len + 1) / 2;
            total += desired - current;
            answers[idx] = total;
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let s = "0001111".to_string();
        let k = 2;
        let queries = vec![vec![0, 6]];
        assert_eq!(
            Solution::count_k_constraint_substrings(s, k, queries),
            vec![26]
        );
    }

    #[test]
    fn example_two() {
        let s = "010101".to_string();
        let k = 1;
        let queries = vec![vec![0, 5], vec![1, 4], vec![2, 3]];
        assert_eq!(
            Solution::count_k_constraint_substrings(s, k, queries),
            vec![15, 9, 3]
        );
    }

    #[test]
    fn all_same_characters() {
        let s = "0000".to_string();
        let k = 1;
        let queries = vec![vec![0, 3], vec![1, 2]];
        assert_eq!(
            Solution::count_k_constraint_substrings(s, k, queries),
            vec![10, 3]
        );
    }

    #[test]
    fn large_k_all_substrings_valid() {
        let s = "0101".to_string();
        let k = 10;
        let queries = vec![vec![0, 3]];
        assert_eq!(
            Solution::count_k_constraint_substrings(s, k, queries),
            vec![10]
        );
    }
}
