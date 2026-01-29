pub struct Solution;

impl Solution {
    /// Finds minimum eating speed to finish all bananas in h hours via binary search.
    ///
    /// # Intuition
    /// The answer is monotonic: if speed k works, any speed > k also works.
    /// Binary search on the speed finds the minimum viable value.
    ///
    /// # Approach
    /// Binary search over speed `[1, max_pile]`. For each speed, compute total
    /// hours as the sum of `ceil(pile / speed)`. If total <= h, try a smaller
    /// speed; otherwise increase speed.
    ///
    /// # Complexity
    /// - Time: O(n * log(max_pile))
    /// - Space: O(1)
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut lo, mut hi) = (1, *piles.iter().max().unwrap_or(&1));
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let hours: i32 = piles.iter().map(|&p| (p + mid - 1) / mid).sum();
            if hours <= h {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn test_tight() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn test_generous_hours() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
