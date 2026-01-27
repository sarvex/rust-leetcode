impl Solution {
    /// Maximum potholes fixable within a budget (each segment of k potholes costs k+1).
    ///
    /// # Intuition
    /// Consecutive pothole segments of length k cost k+1 to fix. Longer segments
    /// yield better value per unit cost, so we prioritize fixing longer segments
    /// first. If a segment cannot be fully fixed, its remainder merges into the
    /// next smaller bucket.
    ///
    /// # Approach
    /// 1. Parse the road string into consecutive pothole segment lengths.
    /// 2. Count segments of each length in a frequency array.
    /// 3. Iterate from longest to shortest, greedily fixing as many full segments
    ///    as the budget allows. Carry unfixed portions to the next smaller bucket.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn max_potholes(road: String, budget: i32) -> i32 {
        let bytes = road.as_bytes();
        let n = bytes.len();
        let mut cnt = vec![0i32; n + 2];
        let mut k = 0usize;

        bytes
            .iter()
            .chain(std::iter::once(&b'.'))
            .for_each(|&b| match b {
                b'x' => k += 1,
                _ => {
                    if k > 0 {
                        cnt[k] += 1;
                        k = 0;
                    }
                }
            });

        let mut ans = 0;
        let mut remaining = budget;

        for k in (1..=n).rev() {
            if remaining == 0 {
                break;
            }
            let cost = k as i32 + 1;
            let t = (remaining / cost).min(cnt[k]);
            ans += t * k as i32;
            remaining -= t * cost;
            cnt[k - 1] += cnt[k] - t;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_road() {
        assert_eq!(Solution::max_potholes("..xxxxx".to_string(), 4), 3);
    }

    #[test]
    fn multiple_segments() {
        assert_eq!(Solution::max_potholes("x]..xx...x..".to_string(), 14), 0);
    }

    #[test]
    fn all_potholes() {
        assert_eq!(Solution::max_potholes("xxx".to_string(), 4), 3);
    }
}
