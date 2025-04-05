impl Solution {
    /// Finds the maximum possible sum of a non-empty subarray after deleting one element
    /// 
    /// #intuition
    /// Use Kadane's algorithm to compute maximum subarrays from both left and right directions.
    /// For each position, consider skipping that element and combining max sums from both sides.
    /// 
    /// #approach
    /// 1. Maintain two arrays for max sums ending at each position from left and right
    /// 2. Use Kadane's algorithm to fill these arrays
    /// 3. For each position i, consider max(left[i-1] + right[i+1]) as potential answer
    /// 4. Compare with the regular max subarray sum without deletion
    /// 
    /// #complexity
    /// Time: O(n) where n is the length of the array
    /// Space: O(n) for the two dynamic programming arrays
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n: usize = arr.len();
        if n == 1 {
            return arr[0];
        }

        // Maximum subarray sums ending at each position from left and right
        let mut max_ending_at: Vec<i32> = vec![0; n];
        let mut max_starting_at: Vec<i32> = vec![0; n];
        
        // Compute maximum subarrays ending at each position from left
        max_ending_at[0] = arr[0];
        let mut curr_sum: i32 = arr[0];
        for i in 1..n {
            curr_sum = (curr_sum.max(0)) + arr[i];
            max_ending_at[i] = curr_sum;
        }
        
        // Compute maximum subarrays starting at each position from right
        max_starting_at[n - 1] = arr[n - 1];
        curr_sum = arr[n - 1];
        for i in (0..n-1).rev() {
            curr_sum = (curr_sum.max(0)) + arr[i];
            max_starting_at[i] = curr_sum;
        }
        
        // Find maximum sum possible by deleting one element
        let mut max_sum: i32 = max_ending_at[0].max(max_ending_at[n - 1]);
        for i in 1..n-1 {
            let sum_without_curr = max_ending_at[i - 1].max(0) + max_starting_at[i + 1].max(0);
            max_sum = max_sum.max(sum_without_curr).max(max_ending_at[i]);
        }
        
        max_sum
    }
}
