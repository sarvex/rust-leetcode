impl Solution {
    /// Order enemies by weighted shortest processing time.
    ///
    /// # Intuition
    /// Each enemy contributes a constant damage rate while alive and requires a fixed number of
    /// attack seconds to kill. This is the classic single-machine scheduling objective of
    /// minimizing the weighted sum of completion times.
    ///
    /// # Approach
    /// - Convert each enemy into a job with processing time
    ///   `time = ceil(health / power)` and weight `damage`.
    /// - Sort by increasing `time / damage` (Smith's rule).
    /// - Accumulate completion times and sum `damage * completion_time`.
    ///
    /// # Complexity
    /// - Time: O(n log n) due to sorting.
    /// - Space: O(n).
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        let power = power as i64;
        let mut enemies: Vec<(i64, i64)> = damage
            .into_iter()
            .zip(health.into_iter())
            .map(|(damage, health)| {
                let time = (health as i64 + power - 1) / power;
                (time, damage as i64)
            })
            .collect();

        enemies.sort_unstable_by(|(time_a, damage_a), (time_b, damage_b)| {
            let left = time_a * damage_b;
            let right = time_b * damage_a;
            left.cmp(&right)
                .then_with(|| damage_b.cmp(damage_a))
                .then_with(|| time_a.cmp(time_b))
        });

        let mut elapsed = 0_i64;
        let mut total = 0_i64;
        for (time, damage) in enemies {
            elapsed += time;
            total += elapsed * damage;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let power = 4;
        let damage = vec![1, 2, 3, 4];
        let health = vec![4, 5, 6, 8];
        assert_eq!(Solution::min_damage(power, damage, health), 39);
    }

    #[test]
    fn example_two() {
        let power = 1;
        let damage = vec![1, 1, 1, 1];
        let health = vec![1, 2, 3, 4];
        assert_eq!(Solution::min_damage(power, damage, health), 20);
    }

    #[test]
    fn example_three() {
        let power = 8;
        let damage = vec![40];
        let health = vec![59];
        assert_eq!(Solution::min_damage(power, damage, health), 320);
    }

    #[test]
    fn equal_ratios_any_order() {
        let power = 3;
        let damage = vec![2, 4];
        let health = vec![3, 6];
        assert_eq!(Solution::min_damage(power, damage, health), 20);
    }

    #[test]
    fn higher_damage_first() {
        let power = 5;
        let damage = vec![10, 1];
        let health = vec![10, 5];
        assert_eq!(Solution::min_damage(power, damage, health), 35);
    }
}
