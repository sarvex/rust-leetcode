impl Solution {
    /// Counts connected components where nodes connect if their LCM ≤ threshold.
    ///
    /// # Intuition
    /// If both a and b divide some m ≤ threshold, then lcm(a,b) ≤ m ≤ threshold.
    /// Numbers exceeding the threshold are isolated since their LCM with anything
    /// also exceeds it.
    ///
    /// # Approach
    /// 1. Partition numbers into isolated (> threshold) and valid (≤ threshold).
    /// 2. Use Union-Find over multiples: for each valid number v, iterate
    ///    multiples v, 2v, 3v, … ≤ threshold and union the first visitor of
    ///    each multiple with v.
    /// 3. Count distinct roots among valid numbers plus the isolated count.
    ///
    /// # Complexity
    /// - Time: O(threshold × log(threshold) × α(n))
    /// - Space: O(threshold)
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let threshold = threshold as usize;

        let (isolated, valid): (Vec<_>, Vec<_>) = nums
            .into_iter()
            .map(|n| n as usize)
            .partition(|&n| n > threshold);

        if valid.is_empty() {
            return isolated.len() as i32;
        }

        let mut parent: Vec<usize> = (0..=threshold).collect();
        let mut first = vec![0usize; threshold + 1];

        for &v in &valid {
            let mut m = v;
            while m <= threshold {
                if first[m] == 0 {
                    first[m] = v;
                } else {
                    let px = Self::find(&mut parent, first[m]);
                    let py = Self::find(&mut parent, v);
                    if px != py {
                        parent[px] = py;
                    }
                }
                m += v;
            }
        }

        let roots: std::collections::HashSet<_> =
            valid.iter().map(|&v| Self::find(&mut parent, v)).collect();

        isolated.len() as i32 + roots.len() as i32
    }

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn shared_multiples_form_components() {
        assert_eq!(Solution::count_components(vec![2, 4, 8, 3, 9], 5), 4);
    }

    #[test]
    fn larger_threshold_merges_more() {
        assert_eq!(Solution::count_components(vec![2, 4, 8, 3, 9, 12], 10), 2);
    }

    #[test]
    fn all_exceed_threshold_are_isolated() {
        assert_eq!(Solution::count_components(vec![100, 200, 300], 5), 3);
    }

    #[test]
    fn single_element_is_one_component() {
        assert_eq!(Solution::count_components(vec![5], 10), 1);
    }

    #[test]
    fn one_divides_everything_connects_all() {
        assert_eq!(Solution::count_components(vec![1, 2, 3], 6), 1);
    }

    #[test]
    fn lcm_exactly_at_threshold_connects() {
        assert_eq!(Solution::count_components(vec![2, 3], 6), 1);
    }

    #[test]
    fn lcm_exceeding_threshold_separates() {
        assert_eq!(Solution::count_components(vec![2, 3], 5), 2);
    }

    #[test]
    fn divisibility_chain_single_component() {
        assert_eq!(Solution::count_components(vec![2, 4, 8], 8), 1);
    }
}
