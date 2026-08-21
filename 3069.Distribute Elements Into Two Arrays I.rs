impl Solution {
    /// Simulates distributing elements into two arrays based on last-element comparison.
    ///
    /// # Intuition
    /// Directly simulate the process: place the first element in arr1, the second in arr2,
    /// then for each subsequent element, compare the last elements of both arrays to decide
    /// placement. Finally, concatenate arr1 and arr2.
    ///
    /// # Approach
    /// 1. Initialize arr1 with nums[0] and arr2 with nums[1].
    /// 2. For each remaining element, compare the last elements of arr1 and arr2.
    /// 3. Append to arr1 if its last element is greater; otherwise append to arr2.
    /// 4. Concatenate arr1 and arr2 to form the result.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = vec![nums[0]];
        let mut arr2 = vec![nums[1]];

        for &num in &nums[2..] {
            if arr1.last().unwrap() > arr2.last().unwrap() {
                arr1.push(num);
            } else {
                arr2.push(num);
            }
        }

        arr1.extend(arr2);
        arr1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::result_array(vec![2, 1, 3]), vec![2, 3, 1]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::result_array(vec![5, 4, 3, 8]),
            vec![5, 3, 4, 8]
        );
    }

    #[test]
    fn test_three_elements_descending() {
        assert_eq!(Solution::result_array(vec![3, 2, 1]), vec![3, 1, 2]);
    }

    #[test]
    fn test_all_to_arr1() {
        // arr1 always has the larger last element
        assert_eq!(
            Solution::result_array(vec![10, 1, 9, 8, 7]),
            vec![10, 9, 8, 7, 1]
        );
    }

    #[test]
    fn test_alternating() {
        assert_eq!(
            Solution::result_array(vec![1, 5, 2, 6, 3]),
            vec![1, 5, 2, 6, 3]
        );
    }
}
