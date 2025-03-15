impl Solution {
    /// Returns a boolean array indicating whether each child will have the greatest number of candies after receiving extra candies.
    ///
    /// # intuition
    /// If a child has the current number of candies plus extra candies greater than or equal to the maximum
    /// number of candies any child has, then they will have the greatest number of candies.
    ///
    /// # approach
    /// 1. Find the maximum number of candies among all children
    /// 2. For each child, check if their current candies plus extra candies is greater than or equal to the maximum
    /// 3. Return a boolean vector with the results
    ///
    /// # complexity
    /// - Time complexity: O(n), where n is the number of children
    /// - Space complexity: O(n) for the result vector
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // Find the maximum number of candies any child has
        let max_candies = match candies.iter().max() {
            Some(&max) => max,
            None => return Vec::new(), // Handle empty input
        };

        // Check for each child if they can have the greatest number of candies
        candies
            .iter()
            .map(|&candy| candy + extra_candies >= max_candies)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_with_candies() {
        let solution = Solution {};
        
        // Test case 1: [2,3,5,1,3], extra_candies = 3
        assert_eq!(
            solution.kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
        
        // Test case 2: [4,2,1,1,2], extra_candies = 1
        assert_eq!(
            solution.kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
        
        // Test case 3: [12,1,12], extra_candies = 10
        assert_eq!(
            solution.kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
        
        // Edge case: empty input
        assert_eq!(solution.kids_with_candies(vec![], 5), Vec::new());
    }
}
