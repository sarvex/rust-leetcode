impl Solution {
    /// Bit manipulation to track seen elements and count common prefix elements.
    ///
    /// # Intuition
    /// Since values are in range [1, n] and n <= 50, we can use two u64 bitmasks
    /// to track which elements have been seen in A and B respectively. At each
    /// index, the count of common elements is the popcount of the AND of both masks.
    ///
    /// # Approach
    /// 1. Maintain two bitmasks `seen_a` and `seen_b`.
    /// 2. For each index i, set the bit for A[i] in `seen_a` and B[i] in `seen_b`.
    /// 3. C[i] = popcount(seen_a & seen_b).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let (mut seen_a, mut seen_b) = (0u64, 0u64);
        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| {
                seen_a |= 1u64 << x;
                seen_b |= 1u64 << y;
                (seen_a & seen_b).count_ones() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
            vec![0, 1, 3]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1], vec![1]),
            vec![1]
        );
    }

    #[test]
    fn test_identical_permutations() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_reversed_permutations() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
            vec![0, 0, 0, 4]
        );
    }

    #[test]
    fn test_max_n() {
        let n = 50;
        let a: Vec<i32> = (1..=n).collect();
        let b: Vec<i32> = (1..=n).rev().collect();
        let result = Solution::find_the_prefix_common_array(a, b);
        assert_eq!(result[0], 0);
        assert_eq!(*result.last().unwrap(), n);
    }
}
