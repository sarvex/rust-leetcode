impl Solution {
    /// Finds the length of the longest substring without repeating characters.
    ///
    /// # Problem
    /// Given a string `s`, find the length of the longest substring without repeating characters.
    ///
    /// # Arguments
    /// * `s` - A string to search for the longest substring without repeating characters
    ///
    /// # Returns
    /// The length of the longest substring without repeating characters as an i32
    ///
    /// # Algorithm
    /// Uses a sliding window approach with two pointers:
    /// 1. Expand the right pointer to include new characters
    /// 2. When a duplicate is found, contract the left pointer until the duplicate is removed
    /// 3. Track the maximum window size seen during this process
    ///
    /// # Complexity
    /// - Time Complexity: O(n) where n is the length of the string
    /// - Space Complexity: O(1) as we use a fixed-size array of 128 characters
    ///
    /// # Example
    /// ```
    /// let s = String::from("abcabcbb");
    /// assert_eq!(Solution::length_of_longest_substring(s), 3);
    /// ```
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_frequency = [0; 128];
        let mut max_length = 0;
        let mut window_start = 0;
        let chars: Vec<char> = s.chars().collect();
        let string_length = chars.len();

        for (window_end, &current_char) in chars.iter().enumerate() {
            char_frequency[current_char as usize] += 1;

            // Shrink window while we have a duplicate character
            while char_frequency[current_char as usize] > 1 {
                let start_char = chars[window_start];
                char_frequency[start_char as usize] -= 1;
                window_start += 1;
            }

            // Update maximum length found so far
            max_length = max_length.max((window_end - window_start + 1) as i32);
        }

        max_length
    }
}
