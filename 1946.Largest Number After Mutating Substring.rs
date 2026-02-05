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
        let bytes = num.into_bytes();

        // Find the first position where mutation is beneficial
        let start = bytes
            .iter()
            .position(|&b| change[(b - b'0') as usize] as u8 + b'0' > b);

        let Some(start) = start else {
            return String::from_utf8(bytes).unwrap();
        };

        // Find where mutation should stop (when mapped < original)
        let end = bytes[start..]
            .iter()
            .position(|&b| change[(b - b'0') as usize] as u8 + b'0' < b)
            .map_or(bytes.len(), |pos| start + pos);

        // Build result: unchanged prefix + mutated segment + unchanged suffix
        let result: Vec<u8> = bytes[..start]
            .iter()
            .copied()
            .chain(bytes[start..end].iter().map(|&b| {
                let digit = (b - b'0') as usize;
                change[digit] as u8 + b'0'
            }))
            .chain(bytes[end..].iter().copied())
            .collect();

        String::from_utf8(result).unwrap()
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
