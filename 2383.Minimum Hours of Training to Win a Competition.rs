impl Solution {
    /// Computes minimum training hours to beat all opponents sequentially.
    ///
    /// # Intuition
    /// Before each opponent, energy and experience must strictly exceed theirs.
    /// Greedily add the minimum training needed at each step.
    ///
    /// # Approach
    /// 1. Iterate through opponents, pairing energy and experience requirements
    /// 2. If current stats are insufficient, add the exact deficit plus one
    /// 3. After each fight, lose energy and gain experience
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_number_of_hours(
        mut energy: i32,
        mut experience: i32,
        energy_cost: Vec<i32>,
        experience_cost: Vec<i32>,
    ) -> i32 {
        energy_cost
            .iter()
            .zip(experience_cost.iter())
            .fold(0, |hours, (&de, &dx)| {
                let mut extra = 0;
                if energy <= de {
                    extra += de + 1 - energy;
                    energy = de + 1;
                }
                if experience <= dx {
                    extra += dx + 1 - experience;
                    experience = dx + 1;
                }
                energy -= de;
                experience += dx;
                hours + extra
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
    }

    #[test]
    fn test_no_training_needed() {
        assert_eq!(
            Solution::min_number_of_hours(100, 100, vec![1, 1], vec![1, 1]),
            0
        );
    }

    #[test]
    fn test_single_opponent() {
        assert_eq!(Solution::min_number_of_hours(1, 1, vec![1], vec![1]), 1);
    }

    #[test]
    fn test_zero_initial_stats() {
        assert_eq!(Solution::min_number_of_hours(0, 0, vec![1], vec![1]), 4);
    }
}
