use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    /// Optimized Dijkstra with bitmask DP and precomputation
    ///
    /// # Intuition
    /// Model as shortest path where state = (people at destination, stage).
    /// Precompute max times and valid groups to avoid redundant calculations.
    ///
    /// # Approach
    /// 1. Precompute max_time for all 2^n subsets using DP on subsets
    /// 2. Precompute all valid groups (size 1 to k) upfront
    /// 3. Use Dijkstra with efficient bit manipulation for returner iteration
    /// 4. Early termination when all people reach destination
    ///
    /// # Complexity
    /// - Time: O(C(n,k) * 2^n * m * n) where C(n,k) is number of valid groups
    /// - Space: O(2^n * m) for distance array plus O(2^n) for precomputation
    pub fn min_time(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let m = m as usize;

        if k == 1 && n > 1 {
            return -1.0;
        }

        let all_mask = (1usize << n) - 1;

        // Precompute max_time for each subset using subset DP
        let max_time_of = (0usize..1 << n).fold(vec![0i32; 1 << n], |mut acc, mask| {
            if mask > 0 {
                let lowest = mask & mask.wrapping_neg();
                let idx = lowest.trailing_zeros() as usize;
                acc[mask] = acc[mask ^ lowest].max(time[idx]);
            }
            acc
        });

        // Precompute valid groups (size 1 to k)
        let valid_groups: Vec<usize> = (1usize..1 << n)
            .filter(|&g| g.count_ones() as usize <= k)
            .collect();

        let mut dist = vec![[f64::INFINITY; 5]; 1 << n];
        dist[0][0] = 0.0;

        let mut heap = BinaryHeap::new();
        heap.push(State(0.0, 0, 0));

        while let Some(State(cost, mask, stage)) = heap.pop() {
            if mask == all_mask {
                return cost;
            }

            if cost > dist[mask][stage] {
                continue;
            }

            let at_base = all_mask ^ mask;

            for &group in &valid_groups {
                if group & at_base != group {
                    continue;
                }

                let cross_time = f64::from(max_time_of[group]) * mul[stage];
                let new_mask = mask | group;
                let new_stage = (stage + (cross_time as usize) % m) % m;

                if new_mask == all_mask {
                    let new_cost = cost + cross_time;
                    if new_cost < dist[new_mask][new_stage] {
                        dist[new_mask][new_stage] = new_cost;
                        heap.push(State(new_cost, new_mask, new_stage));
                    }
                } else {
                    // Iterate only over set bits in new_mask
                    std::iter::successors(Some(new_mask), |&r| {
                        let next = r & (r - 1);
                        if next != 0 { Some(next) } else { None }
                    })
                    .map(|r| r.trailing_zeros() as usize)
                    .for_each(|r| {
                        let ret_time = f64::from(time[r]) * mul[new_stage];
                        let final_mask = new_mask ^ (1 << r);
                        let final_stage = (new_stage + (ret_time as usize) % m) % m;
                        let new_cost = cost + cross_time + ret_time;

                        if new_cost < dist[final_mask][final_stage] {
                            dist[final_mask][final_stage] = new_cost;
                            heap.push(State(new_cost, final_mask, final_stage));
                        }
                    });
                }
            }
        }

        -1.0
    }
}

#[derive(PartialEq)]
struct State(f64, usize, usize);

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.partial_cmp(&self.0).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_person() {
        let result = Solution::min_time(1, 1, 2, vec![5], vec![1.0, 1.3]);
        assert!((result - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_three_people() {
        let result = Solution::min_time(3, 2, 3, vec![2, 5, 8], vec![1.0, 1.5, 0.75]);
        assert!((result - 14.5).abs() < 1e-5);
    }

    #[test]
    fn test_impossible() {
        let result = Solution::min_time(2, 1, 2, vec![10, 10], vec![2.0, 2.0]);
        assert!((result - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn test_two_people_boat_capacity_two() {
        let result = Solution::min_time(2, 2, 2, vec![3, 5], vec![1.0, 1.5]);
        assert!((result - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_stage_advancement() {
        let result = Solution::min_time(2, 2, 3, vec![4, 6], vec![1.0, 2.0, 0.5]);
        assert!((result - 6.0).abs() < 1e-5);
    }
}
