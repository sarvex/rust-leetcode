impl Solution {
    /// Finds the minimum time to repair all cars with mechanics of different ranks
    ///
    /// # intuition
    /// A binary search approach can find the minimum time where all cars can be repaired.
    /// For a given time, each mechanic can repair sqrt(time/rank) cars.
    ///
    /// # approach
    /// 1. Use binary search to find the minimum time needed
    /// 2. For each potential time, check if all cars can be repaired
    /// 3. Return the minimum valid time
    ///
    /// # complexity
    /// Time complexity: O(n * log(m)), where n is the number of mechanics and m is the maximum possible time
    /// Space complexity: O(1)
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars_i64: i64 = cars as i64;
        let min_rank: i64 = *ranks.iter().min().unwrap() as i64;
        
        // Define search boundaries
        let mut left: i64 = 1;
        let mut right: i64 = min_rank * cars_i64 * cars_i64;
        
        while left < right {
            let mid: i64 = left + (right - left) / 2;
            
            if Self::can_repair_all_cars(&ranks, cars_i64, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        
        left
    }
    
    /// Helper function to check if all cars can be repaired within the given time
    fn can_repair_all_cars(ranks: &Vec<i32>, total_cars: i64, time: i64) -> bool {
        let mut cars_repaired: i64 = 0;
        
        for &rank in ranks {
            // Calculate how many cars this mechanic can repair within 'time'
            let rank_i64: i64 = rank as i64;
            let mechanic_capacity: i64 = (time / rank_i64).sqrt();
            cars_repaired += mechanic_capacity;
            
            // Early return if we've already repaired enough cars
            if cars_repaired >= total_cars {
                return true;
            }
        }
        
        false
    }
}
