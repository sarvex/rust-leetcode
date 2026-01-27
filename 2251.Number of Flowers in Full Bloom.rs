use std::collections::BTreeMap;

impl Solution {
    /// Counts flowers in full bloom for each person's arrival time.
    ///
    /// # Intuition
    /// Use a difference array on a BTreeMap to track bloom events. Processing
    /// people in sorted order allows sweeping through events with a running sum.
    ///
    /// # Approach
    /// 1. Build a difference map: +1 at bloom start, -1 at bloom end + 1
    /// 2. Sort people by arrival time while preserving original indices
    /// 3. Sweep through the BTreeMap, updating the running bloom count
    /// 4. For each person, assign the current count before processing later events
    ///
    /// # Complexity
    /// - Time: O(n log n + p log p) where n = flowers, p = people
    /// - Space: O(n + p)
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let n = people.len();

        let mut people: Vec<(usize, i32)> = people.into_iter().enumerate().collect();
        people.sort_unstable_by_key(|&(_, t)| t);

        let mut diff = BTreeMap::new();
        for f in &flowers {
            *diff.entry(f[0]).or_insert(0) += 1;
            *diff.entry(f[1] + 1).or_insert(0) -= 1;
        }

        let mut result = vec![0; n];
        let mut sum = 0;
        let mut person_idx = 0;

        for (k, v) in &diff {
            while person_idx < n && people[person_idx].1 < *k {
                result[people[person_idx].0] = sum;
                person_idx += 1;
            }
            sum += v;
        }

        while person_idx < n {
            result[people[person_idx].0] = sum;
            person_idx += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];
        assert_eq!(
            Solution::full_bloom_flowers(flowers, people),
            vec![1, 2, 2, 2]
        );
    }

    #[test]
    fn test_example_two() {
        let flowers = vec![vec![1, 10], vec![3, 3]];
        let people = vec![3, 3, 2];
        assert_eq!(Solution::full_bloom_flowers(flowers, people), vec![2, 2, 1]);
    }

    #[test]
    fn test_no_flowers_at_time() {
        let flowers = vec![vec![1, 2]];
        let people = vec![3];
        assert_eq!(Solution::full_bloom_flowers(flowers, people), vec![0]);
    }

    #[test]
    fn test_single_flower_single_person() {
        let flowers = vec![vec![5, 5]];
        let people = vec![5];
        assert_eq!(Solution::full_bloom_flowers(flowers, people), vec![1]);
    }
}
