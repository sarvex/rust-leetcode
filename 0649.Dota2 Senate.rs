use std::collections::VecDeque;

impl Solution {
    /// Simulates the Dota2 senate voting using two queues.
    ///
    /// # Intuition
    /// Each senator bans the nearest opposing senator. Using two queues
    /// (one per party), the senator with the smaller index acts first and
    /// re-enters the queue for the next round.
    ///
    /// # Approach
    /// 1. Separate senators into Radiant and Dire queues by index.
    /// 2. Each round, compare fronts; the smaller index wins and re-enters
    ///    with index + n (next round).
    /// 3. The non-empty queue's party wins.
    ///
    /// # Complexity
    /// - Time: O(n) per round, O(n) rounds worst case → O(n²) total
    /// - Space: O(n)
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();
        for (i, b) in senate.bytes().enumerate() {
            if b == b'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i);
            }
        }
        while !radiant.is_empty() && !dire.is_empty() {
            let r = radiant.pop_front().unwrap();
            let d = dire.pop_front().unwrap();
            if r < d {
                radiant.push_back(r + n);
            } else {
                dire.push_back(d + n);
            }
        }
        if radiant.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiant_wins() {
        assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
    }

    #[test]
    fn test_dire_wins() {
        assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
    }
}
