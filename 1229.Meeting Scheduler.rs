impl Solution {
    /// Two-pointer sweep over sorted time slots.
    ///
    /// # Intuition
    /// Sorting both schedules by start time allows a two-pointer approach:
    /// compute the overlap of the current pair of slots, and advance the
    /// pointer with the earlier ending slot. The first overlap of sufficient
    /// duration is the earliest valid meeting time.
    ///
    /// # Approach
    /// 1. Sort both slot arrays by start time
    /// 2. Use two pointers to scan slots in parallel
    /// 3. For each pair, compute overlap `[max(start1,start2), min(end1,end2)]`
    /// 4. If overlap ≥ duration, return `[start, start + duration]`
    /// 5. Advance the pointer whose slot ends earlier
    ///
    /// # Complexity
    /// - Time: O(m log m + n log n) for sorting, O(m + n) for the sweep
    /// - Space: O(1) auxiliary (excluding sort space)
    pub fn min_available_duration(
        mut slots1: Vec<Vec<i32>>,
        mut slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        slots1.sort_unstable_by_key(|s| s[0]);
        slots2.sort_unstable_by_key(|s| s[0]);

        let (mut i, mut j) = (0, 0);

        while i < slots1.len() && j < slots2.len() {
            let start = slots1[i][0].max(slots2[j][0]);
            let end = slots1[i][1].min(slots2[j][1]);

            if end - start >= duration {
                return vec![start, start + duration];
            }

            if slots1[i][1] < slots2[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_slots() {
        assert_eq!(
            Solution::min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                8,
            ),
            vec![60, 68]
        );
    }

    #[test]
    fn no_common_slot() {
        assert_eq!(
            Solution::min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                12,
            ),
            vec![]
        );
    }

    #[test]
    fn exact_fit() {
        assert_eq!(
            Solution::min_available_duration(vec![vec![0, 10]], vec![vec![5, 15]], 5,),
            vec![5, 10]
        );
    }
}
