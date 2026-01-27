impl Solution {
    /// Find minimum time to repair all cars via binary search.
    ///
    /// # Intuition
    /// Each mechanic with rank r repairs cars in r*n² time for n cars. Binary
    /// search on the answer: for a given time, each mechanic can fix sqrt(time/rank) cars.
    ///
    /// # Approach
    /// 1. Binary search on time from 1 to min_rank * cars²
    /// 2. For each candidate time, sum each mechanic's capacity
    /// 3. Return the smallest time where total capacity >= cars
    ///
    /// # Complexity
    /// - Time: O(n * log(min_rank * cars²))
    /// - Space: O(1)
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars_i64 = cars as i64;
        let min_rank = *ranks.iter().min().unwrap_or(&1) as i64;

        let mut left: i64 = 1;
        let mut right: i64 = min_rank * cars_i64 * cars_i64;

        while left < right {
            let mid = left + (right - left) / 2;
            let total: i64 = ranks
                .iter()
                .map(|&r| ((mid / r as i64) as f64).sqrt() as i64)
                .sum();

            if total >= cars_i64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::repair_cars(vec![4, 2, 3, 1], 10), 16);
    }

    #[test]
    fn test_single_mechanic() {
        assert_eq!(Solution::repair_cars(vec![5], 3), 45);
    }

    #[test]
    fn test_uniform_ranks() {
        assert_eq!(Solution::repair_cars(vec![1, 1, 1], 6), 4);
    }
}
