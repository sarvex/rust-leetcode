impl Solution {
    /// Finds the largest number by mutating at most one contiguous substring.
    ///
    /// # Intuition
    /// Greedily mutate the first contiguous segment where the change mapping
    /// produces a larger digit. Stop as soon as the mapping would decrease a digit
    /// after having started mutating.
    ///
    /// # Approach
    /// 1. Scan left to right, skipping digits where change doesn't improve.
    /// 2. Once a beneficial mutation starts, continue while change[d] >= d.
    /// 3. Stop when the mapping would produce a smaller digit.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut bytes = num.into_bytes();
        let mut mutating = false;

        for b in &mut bytes {
            let digit = (*b - b'0') as usize;
            let mapped = change[digit] as u8 + b'0';
            if mutating && mapped < *b {
                break;
            }
            if mapped > *b {
                mutating = true;
                *b = mapped;
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::maximum_number("132".to_string(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
            "832"
        );
    }

    #[test]
    fn test_no_change() {
        assert_eq!(
            Solution::maximum_number("021".to_string(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            "021"
        );
    }
}
