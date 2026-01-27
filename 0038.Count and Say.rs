impl Solution {
    /// Iterative run-length encoding for the count-and-say sequence.
    ///
    /// # Intuition
    /// Each term describes the previous term using run-length encoding:
    /// consecutive identical digits are grouped, and each group becomes
    /// "count" followed by "digit". Building iteratively from the base
    /// case avoids deep recursion.
    ///
    /// # Approach
    /// Start with `[1]`. For each subsequent term, scan the current sequence
    /// with a two-pointer technique to identify runs of identical digits.
    /// Emit the run length and digit value into a new vector. After all
    /// iterations, convert the final vector to a string.
    ///
    /// # Complexity
    /// - Time: O(n × L) — n iterations, L is the length of the current term
    /// - Space: O(L) — storing the current and next term
    pub fn count_and_say(n: i32) -> String {
        (1..n)
            .fold(vec![1u8], |current, _| {
                let mut next = Vec::with_capacity(current.len() * 2);
                let mut slow = 0;
                for fast in 0..=current.len() {
                    if fast == current.len() || current[slow] != current[fast] {
                        next.push((fast - slow) as u8);
                        next.push(current[slow]);
                        slow = fast;
                    }
                }
                next
            })
            .iter()
            .map(|&digit| (digit + b'0') as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn second_term() {
        assert_eq!(Solution::count_and_say(2), "11");
    }

    #[test]
    fn third_term() {
        assert_eq!(Solution::count_and_say(3), "21");
    }

    #[test]
    fn fourth_term() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }

    #[test]
    fn fifth_term() {
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
