impl Solution {
    /// Computes minimum time to collect all garbage with three trucks.
    ///
    /// # Intuition
    /// Each truck collects one type and travels to the last house containing that
    /// type. Total time = pickup time (total chars) + travel time per truck.
    ///
    /// # Approach
    /// 1. Track the last house index for each garbage type (M, P, G)
    /// 2. Sum total garbage units (one second each to pick up)
    /// 3. Compute prefix sums of travel times
    /// 4. Add travel cost for each truck to reach its last stop
    ///
    /// # Complexity
    /// - Time: O(n * m) where m is average string length
    /// - Space: O(1) — only three last-index trackers
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut last = [0usize; 3]; // M=0, P=1, G=2
        let mut total_pickup: i32 = 0;

        for (i, s) in garbage.iter().enumerate() {
            total_pickup += s.len() as i32;
            for &c in s.as_bytes() {
                let idx = match c {
                    b'M' => 0,
                    b'P' => 1,
                    _ => 2,
                };
                last[idx] = i;
            }
        }

        let prefix_travel: Vec<i32> = std::iter::once(0)
            .chain(travel.iter().scan(0, |acc, &t| {
                *acc += t;
                Some(*acc)
            }))
            .collect();

        total_pickup + last.iter().map(|&l| prefix_travel[l]).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::garbage_collection(vec![s("G"), s("P"), s("GP"), s("GG")], vec![2, 4, 3]),
            21
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::garbage_collection(vec![s("MMM"), s("PGM"), s("GP")], vec![3, 10]),
            37
        );
    }

    #[test]
    fn test_single_house() {
        assert_eq!(Solution::garbage_collection(vec![s("MPG")], vec![]), 3);
    }

    #[test]
    fn test_all_at_first_house() {
        assert_eq!(
            Solution::garbage_collection(vec![s("MPG"), s(""), s("")], vec![5, 5]),
            3
        );
    }
}
