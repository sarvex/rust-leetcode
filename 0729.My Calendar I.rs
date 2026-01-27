use std::collections::BTreeMap;

/// Calendar that prevents double-booking using a balanced BST.
///
/// # Intuition
/// Store intervals keyed by end time. For a new booking, check if any
/// existing interval overlaps by querying the tree for the nearest
/// entry after the start time.
///
/// # Approach
/// Use a `BTreeMap<end, start>`. For a new `[start, end)`, look at the
/// first entry with key > start. If that entry's value (start of existing
/// interval) is less than the new end, there is an overlap.
///
/// # Complexity
/// - Time: O(log n) per booking
/// - Space: O(n) for stored intervals
struct MyCalendar {
    intervals: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some((&_end, &existing_start)) = self.intervals.range(start_time + 1..).next() {
            if existing_start < end_time {
                return false;
            }
        }
        self.intervals.insert(end_time, start_time);
        true
    }
}
