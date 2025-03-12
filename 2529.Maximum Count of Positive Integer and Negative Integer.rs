impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // Binary search helper function
        let search = |x: i32| {
            let mut left = 0;
            let mut right = n;
            
            while left < right {
                let mid = (left + right) >> 1;
                if nums[mid] >= x {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            
            left
        };
        
        // Find the index of the first positive number (>= 1)
        let i = search(1);
        // Find the index of the first non-negative number (>= 0)
        let j = search(0);
        
        // Count of positive numbers = total length - index of first positive
        let positive_count = (n - i) as i32;
        // Count of negative numbers = index of first non-negative
        let negative_count = j as i32;
        
        std::cmp::max(positive_count, negative_count)
    }
}
