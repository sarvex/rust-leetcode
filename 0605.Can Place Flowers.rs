impl Solution {
    /// Determines if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.
    ///
    /// # intuition
    /// We can plant a flower at a position if it and its adjacent positions are empty.
    /// By checking each position and its neighbors, we can count how many flowers can be planted.
    ///
    /// # approach
    /// 1. Skip positions efficiently rather than modifying the array
    /// 2. Handle edge cases (first and last positions) separately
    /// 3. Count plantable positions and compare with required count
    ///
    /// # complexity
    /// - Time complexity: O(n), where n is the length of the flowerbed
    /// - Space complexity: O(1), constant extra space
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n <= 0 {
            return true;
        }

        let length = flowerbed.len();
        let n = n as usize;
        let mut count = 0;
        let mut index = 0;

        while index < length && count < n {
            if flowerbed[index] == 0 &&
               (index == 0 || flowerbed[index - 1] == 0) &&
               (index == length - 1 || flowerbed[index + 1] == 0) {
                count += 1;
                if count >= n {
                    return true;
                }
                index += 1;
            }
            index += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
        assert_eq!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0], 3), true);
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2), false);
        assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1], 1), false);
        assert_eq!(Solution::can_place_flowers(vec![], 0), true);
    }
}
