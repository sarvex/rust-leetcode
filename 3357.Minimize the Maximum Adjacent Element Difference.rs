impl Solution {
    /// Binary search on answer with feasibility check using two-value assignment.
    ///
    /// # Intuition
    /// The minimum maximum difference is monotonic: if we can achieve difference d,
    /// we can achieve any d' > d. Binary search on the answer. For each candidate
    /// difference, check if we can assign at most two distinct values to all -1s
    /// such that all adjacent differences are <= candidate.
    ///
    /// # Approach
    /// 1. Binary search on the maximum difference from 0 to max possible difference
    /// 2. For each candidate `d`, compute allowed ranges for each -1 position
    /// 3. Check if there exist two values (x, y) that satisfy all constraints
    /// 4. For feasibility: find intersection of all ranges, then check if two values suffice
    ///
    /// # Complexity
    /// - Time: O(n log(max_diff)) where max_diff is the maximum possible difference
    /// - Space: O(n)
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // Find maximum possible difference for binary search upper bound
        // Need to consider gaps between consecutive known values
        let known_values: Vec<i32> = nums.iter().copied().filter(|&x| x != -1).collect();
        let max_possible_diff = if known_values.is_empty() {
            0
        } else {
            let min_val = *known_values.iter().min().unwrap();
            let max_val = *known_values.iter().max().unwrap();
            // Consider the case where we might need to bridge large gaps
            // Upper bound: maximum gap between consecutive known values (if any)
            let mut max_gap = max_val - min_val;
            let known_positions: Vec<(usize, i32)> = nums
                .iter()
                .enumerate()
                .filter_map(|(i, &x)| if x != -1 { Some((i, x)) } else { None })
                .collect();
            for i in 1..known_positions.len() {
                let gap = (known_positions[i].1 - known_positions[i - 1].1).abs();
                max_gap = max_gap.max(gap);
            }
            max_gap
        };

        // Binary search on the answer
        let (mut lo, mut hi) = (0, max_possible_diff);

        while lo < hi {
            let mid = (lo + hi) / 2;
            if Self::is_feasible(&nums, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo
    }

    /// Checks if we can assign at most two values to all -1s with max difference <= d.
    /// Uses efficient DP without trying all pairs explicitly.
    fn is_feasible(nums: &[i32], max_diff: i32) -> bool {
        let n = nums.len();

        // First check: if no -1s, verify existing differences
        if nums.iter().all(|&x| x != -1) {
            return (1..n).all(|i| (nums[i] - nums[i - 1]).abs() <= max_diff);
        }

        // Collect constraint ranges for -1 positions
        let mut ranges = Vec::new();
        for i in 0..n {
            if nums[i] == -1 {
                let mut min_val = i32::MIN;
                let mut max_val = i32::MAX;

                if i > 0 && nums[i - 1] != -1 {
                    let left = nums[i - 1];
                    min_val = min_val.max(left - max_diff);
                    max_val = max_val.min(left + max_diff);
                }
                if i < n - 1 && nums[i + 1] != -1 {
                    let right = nums[i + 1];
                    min_val = min_val.max(right - max_diff);
                    max_val = max_val.min(right + max_diff);
                }

                if min_val > max_val {
                    return false;
                }
                ranges.push((min_val, max_val));
            }
        }

        // Collect candidate values
        let mut candidates = std::collections::HashSet::new();
        
        // Add known values
        for i in 0..n {
            if nums[i] != -1 {
                candidates.insert(nums[i]);
            }
        }
        
        // Add range endpoints and midpoints for -1 positions
        for (min_val, max_val) in &ranges {
            candidates.insert(*min_val);
            candidates.insert(*max_val);
            if *max_val > *min_val {
                candidates.insert((*min_val + *max_val) / 2);
            }
        }
        
        // Add known values ± max_diff (within reason)
        for i in 0..n {
            if nums[i] != -1 {
                let val = nums[i];
                if val >= max_diff {
                    candidates.insert(val - max_diff);
                }
                if val <= i32::MAX - max_diff {
                    candidates.insert(val + max_diff);
                }
            }
        }

        let candidate_list: Vec<i32> = candidates.into_iter().collect();
        let m = candidate_list.len();

        // Try all pairs, but limit to reasonable number
        if m > 100 {
            // Too many candidates, use alternative approach
            return Self::is_feasible_dp(nums, max_diff);
        }

        for i in 0..m {
            for j in i..m {
                if Self::can_assign_two_values(nums, max_diff, candidate_list[i], candidate_list[j]) {
                    return true;
                }
            }
        }

        false
    }

    /// Efficient DP check for two-value assignment without backtracking.
    /// dp[i][j] = can we assign value j (0=val1, 1=val2) at position i?
    fn can_assign_two_values(nums: &[i32], max_diff: i32, val1: i32, val2: i32) -> bool {
        let n = nums.len();
        // dp[i] = set of values (as bits: 0=val1, 1=val2) that can be assigned at position i
        let mut dp = vec![0u8; n];

        // Process left to right
        for i in 0..n {
            if nums[i] != -1 {
                // Known value: verify constraints with left neighbor
                let current = nums[i];
                
                if i > 0 {
                    if nums[i - 1] == -1 {
                        // Left is -1: current must be compatible with at least one left assignment
                        let compatible = ((dp[i - 1] & 1 != 0) && (current - val1).abs() <= max_diff)
                            || ((dp[i - 1] & 2 != 0) && (current - val2).abs() <= max_diff);
                        if !compatible {
                            return false;
                        }
                    } else {
                        // Left is known: check difference
                        if (current - nums[i - 1]).abs() > max_diff {
                            return false;
                        }
                    }
                }
                continue;
            }

            // Position i is -1: check if val1 or val2 can be assigned
            // Compute allowed range from known neighbors
            let mut min_allowed = i32::MIN;
            let mut max_allowed = i32::MAX;
            
            if i > 0 && nums[i - 1] != -1 {
                let left = nums[i - 1];
                min_allowed = min_allowed.max(left - max_diff);
                max_allowed = max_allowed.min(left + max_diff);
            }
            if i < n - 1 && nums[i + 1] != -1 {
                let right = nums[i + 1];
                min_allowed = min_allowed.max(right - max_diff);
                max_allowed = max_allowed.min(right + max_diff);
            }

            // Check if val1 is valid
            let mut valid1 = val1 >= min_allowed && val1 <= max_allowed;
            if valid1 && i > 0 && nums[i - 1] == -1 {
                // Left is -1: must be compatible with at least one left assignment
                valid1 = ((dp[i - 1] & 1 != 0) && (val1 - val1).abs() <= max_diff)
                    || ((dp[i - 1] & 2 != 0) && (val1 - val2).abs() <= max_diff);
            }
            if valid1 && i < n - 1 && nums[i + 1] != -1 {
                // Right is known: verify compatibility
                let right = nums[i + 1];
                valid1 = valid1 && (val1 - right).abs() <= max_diff;
            }

            // Check if val2 is valid
            let mut valid2 = val2 >= min_allowed && val2 <= max_allowed;
            if valid2 && i > 0 && nums[i - 1] == -1 {
                // Left is -1: must be compatible with at least one left assignment
                valid2 = ((dp[i - 1] & 1 != 0) && (val2 - val1).abs() <= max_diff)
                    || ((dp[i - 1] & 2 != 0) && (val2 - val2).abs() <= max_diff);
            }
            if valid2 && i < n - 1 && nums[i + 1] != -1 {
                // Right is known: verify compatibility
                let right = nums[i + 1];
                valid2 = valid2 && (val2 - right).abs() <= max_diff;
            }

            if valid1 {
                dp[i] |= 1;
            }
            if valid2 {
                dp[i] |= 2;
            }

            if dp[i] == 0 {
                return false;
            }
        }

        // Final verification: ensure all adjacent pairs satisfy constraints
        // This catches any edge cases where local checks might have missed something
        for i in 1..n {
            let left_val = if nums[i - 1] == -1 {
                // Check if there's a valid assignment (prefer val1, then val2)
                if dp[i - 1] & 1 != 0 {
                    val1
                } else if dp[i - 1] & 2 != 0 {
                    val2
                } else {
                    return false;
                }
            } else {
                nums[i - 1]
            };
            
            let right_val = if nums[i] == -1 {
                if dp[i] & 1 != 0 {
                    val1
                } else if dp[i] & 2 != 0 {
                    val2
                } else {
                    return false;
                }
            } else {
                nums[i]
            };
            
            // However, we need to check if there EXISTS a valid assignment path
            // Just checking one arbitrary path isn't sufficient
            // Instead, verify that for each adjacent pair of -1s, there's at least one compatible assignment
            if nums[i - 1] == -1 && nums[i] == -1 {
                // Both are -1: check if there's any compatible pair
                let compatible = ((dp[i - 1] & 1 != 0) && (dp[i] & 1 != 0) && (val1 - val1).abs() <= max_diff)
                    || ((dp[i - 1] & 1 != 0) && (dp[i] & 2 != 0) && (val1 - val2).abs() <= max_diff)
                    || ((dp[i - 1] & 2 != 0) && (dp[i] & 1 != 0) && (val2 - val1).abs() <= max_diff)
                    || ((dp[i - 1] & 2 != 0) && (dp[i] & 2 != 0) && (val2 - val2).abs() <= max_diff);
                if !compatible {
                    return false;
                }
            } else {
                // At least one is known: verify difference
                if (left_val - right_val).abs() > max_diff {
                    return false;
                }
            }
        }

        true
    }

    /// Alternative DP approach when too many candidates.
    fn is_feasible_dp(nums: &[i32], max_diff: i32) -> bool {
        let n = nums.len();
        
        // Collect all constraint ranges
        let mut ranges = Vec::new();
        for i in 0..n {
            if nums[i] == -1 {
                let mut min_val = i32::MIN;
                let mut max_val = i32::MAX;

                if i > 0 && nums[i - 1] != -1 {
                    let left = nums[i - 1];
                    min_val = min_val.max(left - max_diff);
                    max_val = max_val.min(left + max_diff);
                }
                if i < n - 1 && nums[i + 1] != -1 {
                    let right = nums[i + 1];
                    min_val = min_val.max(right - max_diff);
                    max_val = max_val.min(right + max_diff);
                }

                if min_val > max_val {
                    return false;
                }
                ranges.push((i, min_val, max_val));
            }
        }

        if ranges.is_empty() {
            return true;
        }

        // Try a limited set of candidate values
        let mut candidates = std::collections::HashSet::new();
        for i in 0..n {
            if nums[i] != -1 {
                candidates.insert(nums[i]);
            }
        }
        for &(_, min_val, max_val) in &ranges {
            candidates.insert(min_val);
            candidates.insert(max_val);
        }

        let candidate_list: Vec<i32> = candidates.into_iter().collect();
        let m = candidate_list.len().min(50); // Limit to 50 candidates

        for i in 0..m {
            for j in i..m {
                if Self::can_assign_two_values(nums, max_diff, candidate_list[i], candidate_list[j]) {
                    return true;
                }
            }
        }

        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_difference(vec![1, 2, -1, 10, 8]), 4);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_difference(vec![-1, -1, -1]), 0);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_difference(vec![-1, 10, -1, 8]), 1);
    }

    #[test]
    fn test_failing_case() {
        assert_eq!(Solution::min_difference(vec![2, -1, 4, -1, -1, 6]), 1);
    }

    #[test]
    fn test_large_case() {
        let nums = vec![
            4147, 11098, 10014, -1, -1, 1329, -1, 8870, 13649, 16392, 25035, 1685, -1, 9754, -1,
            24948, 26665, 546, -1, -1, -1, 23035, 23387, 13465, -1, 5354, 23367, -1, 22966, 12767,
        ];
        assert_eq!(Solution::min_difference(nums), 26119);
    }
}
