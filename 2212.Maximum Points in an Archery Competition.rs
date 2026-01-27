impl Solution {
    /// Maximize Bob's archery score by choosing sections to beat Alice via bitmask.
    ///
    /// # Intuition
    /// With only 12 scoring sections, enumerate all 2^12 subsets to find which
    /// combination of sections Bob should win to maximize his score within the
    /// arrow budget.
    ///
    /// # Approach
    /// 1. For each bitmask, compute the total arrows needed and score gained.
    /// 2. Track the mask yielding the highest score within the arrow limit.
    /// 3. Reconstruct the arrow allocation, placing leftover arrows in section 0.
    ///
    /// # Complexity
    /// - Time: O(2^m * m) where m = 12
    /// - Space: O(m)
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let m = alice_arrows.len();
        let (mut best_mask, mut max_score) = (0, 0);

        for mask in 1..(1 << m) {
            let (arrows_needed, score) = (0..m)
                .filter(|&i| (mask >> i) & 1 == 1)
                .fold((0, 0), |(a, s), i| (a + alice_arrows[i] + 1, s + i as i32));

            if arrows_needed <= num_arrows && score > max_score {
                max_score = score;
                best_mask = mask;
            }
        }

        let mut result = vec![0; m];
        let mut remaining = num_arrows;
        for i in 0..m {
            if (best_mask >> i) & 1 == 1 {
                result[i] = alice_arrows[i] + 1;
                remaining -= result[i];
            }
        }
        result[0] += remaining;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let result = Solution::maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]);
        assert_eq!(result.iter().sum::<i32>(), 9);
    }

    #[test]
    fn all_arrows_section_zero() {
        let result = Solution::maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(result.iter().sum::<i32>(), 3);
    }
}
