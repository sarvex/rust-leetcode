impl Solution {
    /// Brute-force over all ride pairs in both orderings.
    ///
    /// # Intuition
    /// With n, m ≤ 100 we can try every (land, water) pair in both orders
    /// and take the global minimum finish time.
    ///
    /// # Approach
    /// For each pair (i, j):
    /// - Land-first: start land at its open time, finish, then start water at
    ///   max(land_finish, waterStartTime[j]), add waterDuration[j].
    /// - Water-first: symmetric.
    /// Return the minimum across all candidates.
    ///
    /// # Complexity
    /// - Time: O(n × m)
    /// - Space: O(1)
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let finish = |start_a: i32, dur_a: i32, start_b: i32, dur_b: i32| -> i32 {
            let end_a = start_a + dur_a;
            end_a.max(start_b) + dur_b
        };

        land_start_time
            .iter()
            .zip(land_duration.iter())
            .flat_map(|(&ls, &ld)| {
                water_start_time
                    .iter()
                    .zip(water_duration.iter())
                    .flat_map(move |(&ws, &wd)| [finish(ls, ld, ws, wd), finish(ws, wd, ls, ld)])
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]),
            9
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]),
            14
        );
    }

    #[test]
    fn test_single_pair_land_first_better() {
        // land: start=1, dur=2 → finish=3; water: start=4, dur=1 → finish=5
        // water first: start=4, dur=1 → finish=5; land start=1 ok → finish=5+2=7
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![2], vec![4], vec![1]),
            5
        );
    }

    #[test]
    fn test_wait_before_second_ride() {
        // land: start=10, dur=1 → finish=11; water: start=20, dur=1
        // land first: 10+1=11, wait until 20, finish=21
        // water first: 20+1=21, land opened at 10 → finish=21+1=22
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![20], vec![1]),
            21
        );
    }

    #[test]
    fn test_multiple_rides_min_chosen() {
        // Best plan: land[1] start=1 dur=1 finish=2; water[0] start=1 dur=1 finish=3
        assert_eq!(
            Solution::earliest_finish_time(vec![100, 1], vec![1, 1], vec![1, 100], vec![1, 1]),
            3
        );
    }
}
