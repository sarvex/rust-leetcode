use std::collections::HashMap;

impl Solution {
    /// Greedy bitmask approach for cinema seat allocation.
    ///
    /// # Intuition
    /// Only rows with at least one reserved seat need individual processing.
    /// Rows without any reservation can always fit exactly 2 groups. We encode
    /// reserved seats as a bitmask per row and check three possible blocks
    /// (left, middle, right) using bitwise AND.
    ///
    /// # Approach
    /// 1. Build a HashMap mapping each row (that has reservations) to a bitmask
    ///    of its reserved seats (bits 1–10).
    /// 2. For each affected row, greedily check if the left block (seats 2–5)
    ///    and right block (seats 6–9) are free. If both are free, count 2.
    ///    Otherwise check if the middle block (seats 4–7) is free for 1.
    /// 3. Unaffected rows each contribute 2 groups.
    ///
    /// # Complexity
    /// - Time: O(r) where r = reservedSeats.len() (≤ 10^4)
    /// - Space: O(r) for the HashMap
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut row_mask: HashMap<i32, u16> = HashMap::with_capacity(reserved_seats.len());

        for seat in &reserved_seats {
            *row_mask.entry(seat[0]).or_insert(0) |= 1 << seat[1];
        }

        let left: u16 = 0b0_0011_1100; // seats 2,3,4,5
        let middle: u16 = 0b0_1111_0000; // seats 4,5,6,7
        let right: u16 = 0b11_1100_0000; // seats 6,7,8,9

        let affected = row_mask.len() as i32;
        let mut count = (n - affected) * 2;

        for mask in row_mask.values() {
            let left_free = mask & left == 0;
            let right_free = mask & right == 0;

            if left_free && right_free {
                count += 2;
            } else if left_free || right_free || (mask & middle == 0) {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::max_number_of_families(
                3,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 8],
                    vec![2, 6],
                    vec![3, 1],
                    vec![3, 10]
                ]
            ),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]),
            2
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::max_number_of_families(
                4,
                vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]
            ),
            4
        );
    }

    #[test]
    fn test_single_row_no_reservations() {
        assert_eq!(
            Solution::max_number_of_families(1, vec![vec![1, 1]]),
            2
        );
    }

    #[test]
    fn test_large_n_few_reservations() {
        assert_eq!(
            Solution::max_number_of_families(1_000_000_000, vec![vec![1, 5]]),
            1_999_999_999
        );
    }

    #[test]
    fn test_all_seats_reserved_in_row() {
        let reserved: Vec<Vec<i32>> = (1..=10).map(|s| vec![1, s]).collect();
        assert_eq!(Solution::max_number_of_families(1, reserved), 0);
    }
}
