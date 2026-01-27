impl Solution {
    /// Total distance traveled with main and additional fuel tanks.
    ///
    /// # Intuition
    /// Every 5 liters consumed from the main tank triggers a 1-liter transfer
    /// from the additional tank (if available). Simulate the process.
    ///
    /// # Approach
    /// 1. Consume fuel one liter at a time from the main tank.
    /// 2. Track cumulative consumption; every 5th liter, transfer from additional.
    /// 3. Each liter yields 10 km.
    ///
    /// # Complexity
    /// - Time: O(main_tank + additional_tank)
    /// - Space: O(1)
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut consumed = 0;
        let mut distance = 0;
        while main_tank > 0 {
            consumed += 1;
            main_tank -= 1;
            distance += 10;
            if consumed % 5 == 0 && additional_tank > 0 {
                additional_tank -= 1;
                main_tank += 1;
            }
        }
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transfer() {
        assert_eq!(Solution::distance_traveled(5, 10), 60);
    }

    #[test]
    fn no_additional_fuel() {
        assert_eq!(Solution::distance_traveled(1, 2), 10);
    }
}
