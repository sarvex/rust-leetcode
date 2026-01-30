
impl Solution {
    /// Counts the number of 1s in the first n characters of the magical string.
    ///
    /// # Intuition
    /// The magical string is self-describing: element at index `i` tells how
    /// many times the current digit repeats. Generate the string on-the-fly
    /// and count 1s.
    ///
    /// # Approach
    /// 1. Seed with [1, 2, 2].
    /// 2. Use a read pointer `i` starting at index 2 to determine run lengths.
    /// 3. Alternate between 1 and 2, appending `s[i]` copies each time.
    /// 4. Count 1s in the first `n` elements.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn magical_string(n: i32) -> i32 {
        let n = n as usize;
        let mut s = Vec::with_capacity(n + 2);
        s.extend_from_slice(&[1, 2, 2]);
        let mut i = 2;
        while s.len() < n {
            let prev = *s.last().unwrap();
            let cur = 3 - prev;
            let count = s[i] as usize;
            s.extend((0..count).map(|_| cur));
            i += 1;
        }
        s.iter().take(n).filter(|x| *x == 1).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_six() {
        assert_eq!(Solution::magical_string(6), 3);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::magical_string(1), 1);
    }

    #[test]
    fn test_ten() {
        assert_eq!(Solution::magical_string(10), 4);
    }
}
