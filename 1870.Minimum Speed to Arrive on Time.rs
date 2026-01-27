impl Solution {
    /// Finds the minimum speed to arrive on time using binary search.
    ///
    /// # Intuition
    /// The total travel time is monotonically decreasing with speed.
    /// Binary search over possible speeds to find the minimum that
    /// satisfies the time constraint.
    ///
    /// # Approach
    /// 1. If there are more trains than ceil(hour), return -1.
    /// 2. Binary search over speed in [1, 10^7].
    /// 3. For each candidate speed, compute total time (ceil for all but last).
    /// 4. Return the minimum feasible speed.
    ///
    /// # Complexity
    /// - Time: O(n * log(10^7))
    /// - Space: O(1)
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let n = dist.len();
        if n as f64 > hour.ceil() {
            return -1;
        }

        const MAX_SPEED: i32 = 10_000_000;
        let can_arrive = |speed: i32| -> bool {
            let total: f64 = dist
                .iter()
                .enumerate()
                .map(|(i, &d)| {
                    let t = d as f64 / speed as f64;
                    if i == n - 1 {
                        t
                    } else {
                        t.ceil()
                    }
                })
                .sum();
            total <= hour
        };

        let (mut lo, mut hi) = (1, MAX_SPEED + 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if can_arrive(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        if lo > MAX_SPEED {
            -1
        } else {
            lo
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 6.0), 1);
    }

    #[test]
    fn test_tight_schedule() {
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 2.7), 3);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 1.9), -1);
    }
}
