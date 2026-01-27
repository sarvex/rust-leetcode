use std::collections::BinaryHeap;

impl Solution {
    /// Finds minimum refueling stops to reach target using a greedy max-heap.
    ///
    /// # Intuition
    /// Greedily drive as far as possible. When fuel runs out, retroactively
    /// refuel at the station with the most fuel among all passed stations.
    ///
    /// # Approach
    /// Iterate through stations (including a sentinel at target). Track
    /// remaining fuel. When fuel goes negative, pop the largest fuel from
    /// the max-heap of passed stations. Count each pop as one stop.
    ///
    /// # Complexity
    /// - Time: O(n log n) for heap operations
    /// - Space: O(n) for the heap
    pub fn min_refuel_stops(target: i32, mut fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut stops = 0;
        let mut prev = 0;

        let all_stops = stations.iter().chain(std::iter::once(&vec![target, 0]));

        for station in all_stops {
            let (pos, f) = (station[0], station[1]);
            fuel -= pos - prev;

            while fuel < 0 {
                match heap.pop() {
                    Some(best) => {
                        fuel += best;
                        stops += 1;
                    }
                    None => return -1,
                }
            }

            heap.push(f);
            prev = pos;
        }

        stops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_stops_needed() {
        assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]), -1);
    }

    #[test]
    fn test_multiple_stops() {
        assert_eq!(
            Solution::min_refuel_stops(
                100,
                10,
                vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]],
            ),
            2
        );
    }
}
