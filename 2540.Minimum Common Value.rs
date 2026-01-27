impl Solution {
    /// Finds the minimum common value in two sorted arrays using two pointers.
    ///
    /// # Intuition
    /// Since both arrays are sorted, advance the pointer with the smaller value
    /// until a match is found or one array is exhausted.
    ///
    /// # Approach
    /// Two-pointer scan: compare elements and advance the smaller one.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(1)
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);

        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                std::cmp::Ordering::Equal => return nums1[i],
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_exists() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
    }

    #[test]
    fn test_no_common() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![4, 5, 6]), -1);
    }

    #[test]
    fn test_first_element_match() {
        assert_eq!(Solution::get_common(vec![1, 2], vec![1, 3]), 1);
    }

    #[test]
    fn test_single_elements() {
        assert_eq!(Solution::get_common(vec![5], vec![5]), 5);
    }
}
