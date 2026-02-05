impl Solution {
    /// Counts valid ZigZag arrays using cumulative DP.
    ///
    /// # Intuition
    /// Instead of storing individual counts and computing prefix/suffix sums each step,
    /// maintain cumulative sums directly in the DP array. Forward pass accumulates
    /// left-to-right for "going up" transitions, backward pass accumulates right-to-left
    /// for "going down" transitions.
    ///
    /// # Approach
    /// Use two direction arrays where each cell stores the cumulative count of valid
    /// arrays that can reach that position. Alternate between two buffers to avoid
    /// allocation. Use conditional subtraction instead of modulo for speed.
    ///
    /// # Complexity
    /// - Time: O(n * m) where m = r - l + 1
    /// - Space: O(m)
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MODULO: i64 = 1_000_000_007;
        let num_values = (r - l + 1) as usize;
        let array_length = n as usize;

        if array_length == 1 {
            return num_values as i32;
        }

        const INCREASING: usize = 0;
        const DECREASING: usize = 1;

        // cumulative_counts[value][direction]: running sum of valid arrays
        let mut even_step_counts = vec![[1i64; 2]; num_values];
        let mut odd_step_counts = vec![[1i64; 2]; num_values];

        // Cannot increase from minimum value, cannot decrease from maximum value
        even_step_counts[0][INCREASING] = 0;
        odd_step_counts[0][INCREASING] = 0;
        even_step_counts[num_values - 1][DECREASING] = 0;
        odd_step_counts[num_values - 1][DECREASING] = 0;

        for step in 1..array_length {
            let (current_counts, previous_counts) = if step % 2 == 1 {
                (&mut odd_step_counts, &even_step_counts)
            } else {
                (&mut even_step_counts, &odd_step_counts)
            };

            // Forward pass: accumulate increasing transitions (must come from decreasing)
            (1..num_values).for_each(|value| {
                let mut sum =
                    current_counts[value - 1][INCREASING] + previous_counts[value - 1][DECREASING];
                if sum >= MODULO {
                    sum -= MODULO;
                }
                current_counts[value][INCREASING] = sum;
            });

            // Backward pass: accumulate decreasing transitions (must come from increasing)
            (0..num_values - 1).rev().for_each(|value| {
                let mut sum =
                    current_counts[value + 1][DECREASING] + previous_counts[value + 1][INCREASING];
                if sum >= MODULO {
                    sum -= MODULO;
                }
                current_counts[value][DECREASING] = sum;
            });
        }

        let final_counts = if (array_length - 1) % 2 == 1 {
            &odd_step_counts
        } else {
            &even_step_counts
        };

        final_counts
            .iter()
            .flat_map(|counts| counts.iter())
            .fold(0i64, |total, &count| (total + count) % MODULO) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_range() {
        assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
    }

    #[test]
    fn test_medium_range() {
        assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
    }

    #[test]
    fn test_longer_array() {
        assert_eq!(Solution::zig_zag_arrays(4, 1, 3), 16);
    }

    #[test]
    fn test_minimum_constraints() {
        assert_eq!(Solution::zig_zag_arrays(3, 1, 2), 2);
    }

    #[test]
    fn test_larger_range() {
        assert_eq!(Solution::zig_zag_arrays(3, 1, 5), 60);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::zig_zag_arrays(1, 1, 5), 5);
    }

    #[test]
    fn test_max_constraints() {
        let result = Solution::zig_zag_arrays(2000, 1, 2000);
        assert!(result >= 0);
    }
}
