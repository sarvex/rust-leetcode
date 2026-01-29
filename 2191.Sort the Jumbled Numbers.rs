impl Solution {
    /// Sort numbers by their mapped values using a digit-mapping array.
    ///
    /// # Intuition
    /// Transform each number by replacing every digit via the mapping, then
    /// sort by the transformed values while preserving relative order for ties.
    ///
    /// # Approach
    /// 1. Define a mapping function that replaces each digit and reconstructs the number.
    /// 2. Pair each number with its mapped value and original index.
    /// 3. Sort by mapped value (stable sort preserves original order for ties).
    /// 4. Extract the original numbers in sorted order.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting, O(n * d) for mapping where d is digit count
    /// - Space: O(n)
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let map_value = |x: i32| -> i32 {
            if x == 0 {
                return mapping[0];
            }
            let mut y = 0;
            let mut k = 1;
            let mut num = x;
            while num != 0 {
                y += k * mapping[(num % 10) as usize];
                k *= 10;
                num /= 10;
            }
            y
        };

        let mut indexed: Vec<(i32, usize)> = nums
            .iter()
            .enumerate()
            .map(|(i, v)| (map_value(*v), i))
            .collect();
        indexed.sort();

        indexed.iter().map(|(_, i)| nums[*i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_mapping() {
        assert_eq!(
            Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![3, 1, 2]),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn reversed_mapping() {
        assert_eq!(
            Solution::sort_jumbled(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            ),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
    }
}
