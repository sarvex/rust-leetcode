use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Finds the minimum number of meeting rooms required using a min-heap.
    ///
    /// # Intuition
    /// Sort meetings by start time. Use a min-heap to track the earliest ending
    /// meeting. If a new meeting starts after the earliest ends, reuse that room.
    ///
    /// # Approach
    /// 1. Sort intervals by start time.
    /// 2. Push the first meeting's end time into a min-heap.
    /// 3. For each subsequent meeting, if it starts after the heap's min end time,
    ///    pop that room and push the new end time. Otherwise, add a new room.
    /// 4. The heap size is the answer.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n) for the heap
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|i| i[0]);
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(intervals[0][1]));

        for interval in intervals.iter().skip(1) {
            if let Some(&Reverse(earliest_end)) = heap.peek() {
                if earliest_end <= interval[0] {
                    heap.pop();
                }
            }
            heap.push(Reverse(interval[1]));
        }

        heap.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_rooms_needed() {
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            2
        );
    }

    #[test]
    fn one_room_sufficient() {
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]),
            1
        );
    }
}
