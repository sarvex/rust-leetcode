impl Solution {
    /// Finds max chunks for a permutation of 0..n using running maximum.
    ///
    /// # Intuition
    /// In a permutation of `[0, n)`, a chunk boundary exists at index `i`
    /// if the maximum value seen so far equals `i`, meaning all values
    /// `0..=i` are contained in the prefix.
    ///
    /// # Approach
    /// Track the running maximum. Increment the chunk count whenever the
    /// maximum equals the current index.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        arr.iter()
            .enumerate()
            .scan(0, |max_so_far, (i, &val)| {
                *max_so_far = (*max_so_far).max(val);
                Some((*max_so_far == i as i32) as i32)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![0, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn test_reversed() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    }

    #[test]
    fn test_one_swap() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![0]), 1);
    }
}
