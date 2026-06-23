impl Solution {
    /// Counting sort greedy to maximize ice cream bars purchased.
    ///
    /// # Intuition
    /// Buying cheapest bars first maximizes count. Counting sort gives O(n +
    /// max_cost) time; truncating the frequency array to actual max cost avoids
    /// scanning unused slots.
    ///
    /// # Approach
    /// 1. Build a frequency array while tracking the maximum cost seen.
    /// 2. Truncate the array to `max_cost + 1` to bound the scan.
    /// 3. Walk from cost 1 upward; at each price point buy as many bars as
    ///    coins allow, then break once coins are exhausted.
    ///
    /// # Complexity
    /// - Time: O(n + max_cost)
    /// - Space: O(max_cost)
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut freq = vec![0usize; 100_001];
        let mut max_cost = 0usize;

        for cost in &costs {
            let c = *cost as usize;
            freq[c] += 1;
            if c > max_cost {
                max_cost = c;
            }
        }

        freq.truncate(max_cost + 1);

        let mut count = 0i32;
        for (cost, &available) in freq.iter().enumerate().skip(1) {
            if coins < cost as i32 {
                break;
            }
            if available == 0 {
                continue;
            }
            let total_cost = (available * cost) as i32;
            if total_cost > coins {
                count += coins / cost as i32;
                break;
            }
            coins -= total_cost;
            count += available as i32;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
    }

    #[test]
    fn test_single_bar_affordable() {
        assert_eq!(Solution::max_ice_cream(vec![5], 5), 1);
    }

    #[test]
    fn test_single_bar_not_affordable() {
        assert_eq!(Solution::max_ice_cream(vec![6], 5), 0);
    }

    #[test]
    fn test_all_same_cost() {
        assert_eq!(Solution::max_ice_cream(vec![3, 3, 3, 3], 9), 3);
    }

    #[test]
    fn test_exact_budget() {
        assert_eq!(Solution::max_ice_cream(vec![1, 2, 3, 4], 10), 4);
    }

    #[test]
    fn test_large_coins_buys_all() {
        assert_eq!(
            Solution::max_ice_cream(vec![100_000, 100_000], 100_000_000),
            2
        );
    }
}
