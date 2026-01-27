impl Solution {
    /// Finds the smallest letter strictly greater than target via binary search.
    ///
    /// # Intuition
    /// The sorted letters array wraps around, so if no letter is strictly
    /// greater, return the first letter.
    ///
    /// # Approach
    /// Binary search for the first letter greater than target. If the search
    /// lands past the end, wrap to index 0.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut lo, mut hi) = (0, letters.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if letters[mid] > target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        letters[lo % letters.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
    }

    #[test]
    fn test_wrap_around() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j'),
            'c'
        );
    }

    #[test]
    fn test_middle() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'),
            'f'
        );
    }

    #[test]
    fn test_exact_match() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
    }
}
