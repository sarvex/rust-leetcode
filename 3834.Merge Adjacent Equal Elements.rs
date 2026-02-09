impl Solution {
    /// Stack-based left-to-right merge of adjacent equal elements.
    ///
    /// # Intuition
    /// Merging the leftmost equal pair is equivalent to processing left-to-right and combining
    /// each new element with the top of a stack while they are equal (repeatedly).
    ///
    /// # Approach
    /// Maintain a stack. For each element: while the stack top equals the current value, pop
    /// and double; then push. The stack is the final array.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut st = Vec::new();
        for x in nums {
            let mut v = x as i64;
            while let Some(&last) = st.last() {
                if last == v {
                    st.pop();
                    v *= 2;
                } else {
                    break;
                }
            }
            st.push(v);
        }
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::merge_adjacent(vec![3, 1, 1, 2]), vec![3, 4]);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::merge_adjacent(vec![2, 2, 4]), vec![8]);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::merge_adjacent(vec![3, 7, 5]), vec![3, 7, 5]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::merge_adjacent(vec![1]), vec![1]);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(Solution::merge_adjacent(vec![2, 2, 2, 2]), vec![8]);
    }
}
