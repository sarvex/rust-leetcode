impl Solution {
    /// Single pass: simulate HP from start 0 and count valid starts via binary search.
    ///
    /// # Intuition
    /// For room `i`, a run starting at `start` earns a point iff remaining HP
    /// `hp - (P[i+1] - P[start])` ≥ `requirement[i]`, i.e. `P[start] ≥ P[i+1] - (hp - r)`.
    /// If the run from start 0 earns a point here, all starts 0..=i do; else binary
    /// search the prefix array for the count with `P[start] ≥ threshold`.
    ///
    /// # Approach
    /// One loop: maintain cumulative damage in `damage_sum` (prefix[1..=i]) and current
    /// `hp` after damage so far. If `hp ≥ r` add `index + 1`; else use `r - hp` as
    /// threshold and add `index - partition_point` over `damage_sum`.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn total_score(hp: i32, damage: Vec<i32>, requirement: Vec<i32>) -> i64 {
        let n = damage.len();
        let mut damage_sum = Vec::with_capacity(n);
        let mut hp = hp as i64;
        let mut result = 0i64;

        for (index, (d, r)) in damage.iter().zip(requirement.iter()).enumerate() {
            let (d, r) = (*d as i64, *r as i64);
            hp -= d;
            if hp >= r {
                result += index as i64 + 1;
            } else {
                let insufficient = r - hp;
                let start = damage_sum.partition_point(|&p| p < insufficient);
                if start != damage_sum.len() {
                    result += (index - start) as i64;
                }
            }
            let last = damage_sum.last().copied().unwrap_or(0);
            damage_sum.push(last + d);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::total_score(11, vec![3, 6, 7], vec![4, 2, 5]),
            3
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::total_score(2, vec![10000, 1], vec![1, 1]),
            1
        );
    }

    #[test]
    fn test_single_room_earn() {
        assert_eq!(Solution::total_score(10, vec![2], vec![5]), 1);
    }

    #[test]
    fn test_single_room_no_earn() {
        assert_eq!(Solution::total_score(2, vec![5], vec![5]), 0);
    }

    #[test]
    fn test_all_earn() {
        assert_eq!(
            Solution::total_score(100, vec![1, 2, 3], vec![1, 1, 1]),
            6
        );
    }
}
