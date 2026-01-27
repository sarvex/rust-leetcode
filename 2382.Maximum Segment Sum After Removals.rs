impl Solution {
    /// Computes maximum segment sum after each removal using reverse Union-Find.
    ///
    /// # Intuition
    /// Processing removals directly is inefficient. Reversing the process — starting
    /// empty and adding elements back — allows efficient segment merging via Union-Find.
    ///
    /// # Approach
    /// 1. Use Union-Find with path compression
    /// 2. Process queries in reverse order (additions instead of removals)
    /// 3. For each addition, merge with adjacent segments if present
    /// 4. Track maximum segment sum at each step, build answer in reverse
    ///
    /// # Complexity
    /// - Time: O(n × α(n)) ≈ O(n)
    /// - Space: O(n)
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut segment_sum: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let mut present = vec![false; n];
        let mut answer = vec![0i64; n];
        let mut max_sum: i64 = 0;

        for i in (1..n).rev() {
            let idx = remove_queries[i] as usize;
            present[idx] = true;

            if idx > 0 && present[idx - 1] {
                Self::union(&mut parent, &mut segment_sum, idx - 1, idx);
            }
            if idx + 1 < n && present[idx + 1] {
                Self::union(&mut parent, &mut segment_sum, idx + 1, idx);
            }

            let root = Self::find(&mut parent, idx);
            max_sum = max_sum.max(segment_sum[root]);
            answer[i - 1] = max_sum;
        }

        answer
    }

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], segment_sum: &mut [i64], x: usize, y: usize) {
        let (rx, ry) = (Self::find(parent, x), Self::find(parent, y));
        if rx != ry {
            parent[rx] = ry;
            segment_sum[ry] += segment_sum[rx];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1]),
            vec![14, 7, 2, 2, 0]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::maximum_segment_sum(vec![3, 2, 11, 1], vec![3, 2, 1, 0]),
            vec![16, 5, 3, 0]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_segment_sum(vec![5], vec![0]), vec![0]);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(
            Solution::maximum_segment_sum(vec![1, 2], vec![0, 1]),
            vec![2, 0]
        );
    }

    #[test]
    fn test_remove_middle_first() {
        assert_eq!(
            Solution::maximum_segment_sum(vec![1, 10, 1], vec![1, 0, 2]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn test_large_values() {
        assert_eq!(
            Solution::maximum_segment_sum(
                vec![1_000_000_000, 1_000_000_000, 1_000_000_000],
                vec![1, 0, 2]
            ),
            vec![1_000_000_000, 1_000_000_000, 0]
        );
    }
}
