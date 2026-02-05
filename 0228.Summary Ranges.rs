impl Solution {
    /// Summarizes consecutive number ranges from a sorted array.
    ///
    /// # Intuition
    /// Track the start of each range. When a gap is found, close the current
    /// range and start a new one.
    ///
    /// # Approach
    /// 1. Track the start of the current range.
    /// 2. When `nums[i] != nums[i-1] + 1`, output the range `[start, nums[i-1]]`.
    /// 3. Handle the final range after the loop.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) excluding output
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return Vec::new();
        }

        let (mut result, start) =
            nums.windows(2)
                .fold((Vec::new(), nums[0]), |(mut result, start), w| {
                    if w[1] != w[0] + 1 {
                        Self::push_range(&mut result, start, w[0]);
                        (result, w[1])
                    } else {
                        (result, start)
                    }
                });
        Self::push_range(&mut result, start, *nums.last().unwrap());
        result
    }

    fn push_range(result: &mut Vec<String>, start: i32, end: i32) {
        if start == end {
            result.push(start.to_string());
        } else {
            result.push(format!("{}->{}", start, end));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_ranges() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }

    #[test]
    fn single_elements() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
    }

    #[test]
    fn empty_input() {
        assert!(Solution::summary_ranges(vec![]).is_empty());
    }
}
