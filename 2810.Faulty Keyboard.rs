use std::collections::VecDeque;

impl Solution {
    /// Simulate a faulty keyboard where 'i' reverses current text.
    ///
    /// # Intuition
    /// Instead of reversing repeatedly, use a deque and a direction flag.
    /// On 'i', flip the direction; otherwise push to the front or back based
    /// on current direction. Collect in the correct order at the end.
    ///
    /// # Approach
    /// 1. Maintain a `VecDeque` and a boolean `front` flag.
    /// 2. For each character: if 'i', toggle the flag; else push to front/back.
    /// 3. Collect characters respecting the final direction.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn final_string(s: String) -> String {
        let mut deque = VecDeque::with_capacity(s.len());
        let mut push_back = true;
        for b in s.bytes() {
            match b {
                b'i' => push_back = !push_back,
                _ => match push_back {
                    true => deque.push_back(b),
                    false => deque.push_front(b),
                },
            }
        }
        let bytes: Vec<u8> = match push_back {
            true => deque.into_iter().collect(),
            false => deque.into_iter().rev().collect(),
        };
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_reversal() {
        assert_eq!(Solution::final_string("string".to_string()), "rtsng");
    }

    #[test]
    fn multiple_reversals() {
        assert_eq!(Solution::final_string("poiinter".to_string()), "ponter");
    }

    #[test]
    fn no_i_characters() {
        assert_eq!(Solution::final_string("abc".to_string()), "abc");
    }
}
