impl Solution {
    /// Maximize simultaneous running time of n computers using battery redistribution.
    ///
    /// # Intuition
    /// Sort batteries and greedily distribute extra capacity from smaller batteries
    /// upward to level the running time across all n computers.
    ///
    /// # Approach
    /// 1. Sort batteries and sum the extra batteries (those not directly assigned).
    /// 2. Walk from the smallest assigned battery upward, distributing extra capacity
    ///    evenly across assigned slots until exhausted or a level boundary is reached.
    ///
    /// # Complexity
    /// - Time: O(m log m) where m is the number of batteries
    /// - Space: O(1) auxiliary beyond sorting
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut batteries = batteries;
        let m = batteries.len();
        batteries.sort_unstable();

        let extra_start = m - n as usize;
        let mut extra_sum: i64 = batteries[..extra_start].iter().map(|&b| b as i64).sum();

        let mut i = extra_start;
        let mut cur_height = batteries[i];
        let mut result = cur_height as i64;

        while extra_sum != 0 {
            if i + 1 == m {
                result += extra_sum / n as i64;
                break;
            }

            if batteries[i] == batteries[i + 1] {
                i += 1;
                continue;
            }

            let slots = (i - extra_start + 1) as i64;
            let diff = extra_sum / slots;

            if cur_height as i64 + diff <= batteries[i + 1] as i64 {
                result = cur_height as i64 + diff;
                break;
            }

            extra_sum -= (batteries[i + 1] - batteries[i]) as i64 * slots;
            result = batteries[i + 1] as i64;

            i += 1;
            if i < m {
                cur_height = batteries[i];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_batteries() {
        assert_eq!(Solution::max_run_time(2, vec![3, 3, 3]), 4);
    }

    #[test]
    fn single_computer() {
        assert_eq!(Solution::max_run_time(1, vec![1, 1, 1, 1]), 4);
    }

    #[test]
    fn large_gap() {
        assert_eq!(Solution::max_run_time(2, vec![1, 1, 1, 1, 1, 100]), 100);
    }
}
