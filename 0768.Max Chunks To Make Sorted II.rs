impl Solution {
    /// Finds max chunks to independently sort and reconstruct the sorted array.
    ///
    /// # Intuition
    /// A chunk boundary exists where all elements to the left are less than or
    /// equal to all elements to the right. A monotonic stack of chunk maximums
    /// merges chunks when a smaller element is encountered.
    ///
    /// # Approach
    /// Maintain a stack of chunk maximums. For each element, if it is smaller
    /// than the stack top, merge chunks by popping until the invariant holds,
    /// preserving the overall maximum. Push the element or the merged maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the stack
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        arr.iter()
            .fold(Vec::new(), |mut stack, &val| {
                if stack.last().is_some_and(|&top| top > val) {
                    let max_val = stack.pop().unwrap();
                    while stack.last().is_some_and(|&top| top > val) {
                        stack.pop();
                    }
                    stack.push(max_val);
                } else {
                    stack.push(val);
                }
                stack
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn test_multiple_chunks() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 1, 1]), 3);
    }
}
