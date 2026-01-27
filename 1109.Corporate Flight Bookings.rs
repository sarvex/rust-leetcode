impl Solution {
    /// Computes total seats booked per flight using a difference array.
    ///
    /// # Intuition
    /// A difference array efficiently handles range updates. Each booking
    /// adds seats to a range. The prefix sum gives the actual seat count.
    ///
    /// # Approach
    /// For each booking `[first, last, seats]`, increment `diff[first-1]`
    /// and decrement `diff[last]`. Compute prefix sums for the result.
    ///
    /// # Complexity
    /// - Time: O(n + m) where m is booking count
    /// - Space: O(n) for the result
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diff = vec![0i32; n + 1];
        for b in &bookings {
            diff[(b[0] - 1) as usize] += b[2];
            diff[b[1] as usize] -= b[2];
        }
        let mut result = Vec::with_capacity(n);
        let mut acc = 0;
        for i in 0..n {
            acc += diff[i];
            result.push(acc);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
            vec![10, 55, 45, 25, 25]
        );
    }

    #[test]
    fn test_single_booking() {
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 2, 10]], 2),
            vec![10, 10]
        );
    }
}
