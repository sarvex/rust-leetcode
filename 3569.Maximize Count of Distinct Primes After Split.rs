/// Highly Optimized Solution with Sorted Vectors and Segment Tree
///
/// # Intuition
/// answer[k] = total_primes + count(primes with first < k <= last)
/// Use interval coverage with segment tree for range add / max query.
///
/// # Approach
/// 1. Sorted Vec per prime instead of BTreeSet (better cache locality)
/// 2. Segment tree with lazy propagation for range add and max query
/// 3. u32 types to reduce memory bandwidth
/// 4. HashMap only for primes that actually appear
///
/// # Complexity
/// - Time: O((n + q) * log n) amortized
/// - Space: O(n + distinct_primes)
use std::collections::HashMap;

struct SegTree {
    n: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let size = 4 * n.max(1);
        Self {
            n,
            tree: vec![0; size],
            lazy: vec![0; size],
        }
    }

    #[inline(always)]
    fn push(&mut self, node: usize) {
        if self.lazy[node] != 0 {
            let l = self.lazy[node];
            self.tree[node * 2] += l;
            self.lazy[node * 2] += l;
            self.tree[node * 2 + 1] += l;
            self.lazy[node * 2 + 1] += l;
            self.lazy[node] = 0;
        }
    }

    fn update(&mut self, l: usize, r: usize, val: i32) {
        if self.n == 0 || l > r || r >= self.n {
            return;
        }
        self.update_inner(1, 0, self.n - 1, l, r, val);
    }

    fn update_inner(&mut self, node: usize, tl: usize, tr: usize, l: usize, r: usize, val: i32) {
        if l > tr || r < tl {
            return;
        }
        if l <= tl && tr <= r {
            self.tree[node] += val;
            self.lazy[node] += val;
            return;
        }
        self.push(node);
        let mid = (tl + tr) / 2;
        self.update_inner(node * 2, tl, mid, l, r, val);
        self.update_inner(node * 2 + 1, mid + 1, tr, l, r, val);
        self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
    }

    #[inline(always)]
    fn max(&self) -> i32 {
        if self.n == 0 { 0 } else { self.tree[1] }
    }
}

const MAX_VAL: usize = 100_001;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 {
            return vec![0; queries.len()];
        }

        let is_prime = sieve();
        let mut nums = nums;
        let mut positions: HashMap<u32, Vec<u32>> = HashMap::with_capacity(1000);
        let mut total_primes = 0i32;

        nums.iter().enumerate().for_each(|(i, &num)| {
            let v = num as u32;
            if is_prime[v as usize] {
                let pos = positions.entry(v).or_default();
                if pos.is_empty() {
                    total_primes += 1;
                }
                pos.push(i as u32);
            }
        });

        let mut seg = SegTree::new(n - 1);

        positions
            .values()
            .filter(|pos| pos.len() > 1)
            .for_each(|pos| {
                let f = pos[0] as usize;
                let l = *pos.last().unwrap() as usize;
                seg.update(f, l - 1, 1);
            });

        let mut result = Vec::with_capacity(queries.len());

        for query in &queries {
            let idx = query[0] as u32;
            let new_val = query[1] as u32;
            let old_val = nums[idx as usize] as u32;

            if old_val == new_val {
                result.push(total_primes + seg.max());
                continue;
            }

            // Remove old value
            if is_prime[old_val as usize] {
                if let Some(pos) = positions.get_mut(&old_val) {
                    let old_f = pos[0] as usize;
                    let old_l = *pos.last().unwrap() as usize;

                    if old_l > old_f {
                        seg.update(old_f, old_l - 1, -1);
                    }

                    if let Ok(i) = pos.binary_search(&idx) {
                        pos.remove(i);
                    }

                    if pos.is_empty() {
                        total_primes -= 1;
                        positions.remove(&old_val);
                    } else {
                        let new_f = pos[0] as usize;
                        let new_l = *pos.last().unwrap() as usize;
                        if new_l > new_f {
                            seg.update(new_f, new_l - 1, 1);
                        }
                    }
                }
            }

            nums[idx as usize] = new_val as i32;

            // Add new value
            if is_prime[new_val as usize] {
                let pos = positions.entry(new_val).or_default();
                let was_empty = pos.is_empty();

                if pos.len() > 1 {
                    let old_f = pos[0] as usize;
                    let old_l = *pos.last().unwrap() as usize;
                    if old_l > old_f {
                        seg.update(old_f, old_l - 1, -1);
                    }
                }

                let insert_pos = pos.partition_point(|&x| x < idx);
                pos.insert(insert_pos, idx);

                if was_empty {
                    total_primes += 1;
                }

                if pos.len() > 1 {
                    let new_f = pos[0] as usize;
                    let new_l = *pos.last().unwrap() as usize;
                    if new_l > new_f {
                        seg.update(new_f, new_l - 1, 1);
                    }
                }
            }

            result.push(total_primes + seg.max());
        }

        result
    }
}

fn sieve() -> [bool; MAX_VAL] {
    let mut is_prime = [true; MAX_VAL];
    is_prime[0] = false;
    is_prime[1] = false;
    (2..)
        .take_while(|&i| i * i < MAX_VAL)
        .filter(|&i| is_prime[i])
        .for_each(|i| {
            (i * i..MAX_VAL).step_by(i).for_each(|j| {
                is_prime[j] = false;
            });
        });
    is_prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::maximum_count(vec![2, 1, 3, 1, 2], vec![vec![1, 2], vec![3, 3]]),
            vec![3, 4]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::maximum_count(vec![2, 1, 4], vec![vec![0, 1]]),
            vec![0]
        );
    }

    #[test]
    fn single_prime() {
        assert_eq!(
            Solution::maximum_count(vec![2, 2], vec![vec![0, 3]]),
            vec![2]
        );
    }

    #[test]
    fn no_primes() {
        assert_eq!(
            Solution::maximum_count(vec![1, 4, 6, 8], vec![vec![0, 1], vec![1, 1]]),
            vec![0, 0]
        );
    }

    #[test]
    fn all_same_prime() {
        assert_eq!(
            Solution::maximum_count(vec![2, 2, 2, 2], vec![vec![1, 3]]),
            vec![3]
        );
    }

    #[test]
    fn multiple_updates() {
        assert_eq!(
            Solution::maximum_count(vec![2, 3, 5], vec![vec![1, 7], vec![1, 3], vec![1, 2]]),
            vec![3, 3, 3]
        );
    }

    #[test]
    fn edge_n2() {
        assert_eq!(
            Solution::maximum_count(vec![2, 3], vec![vec![0, 3], vec![1, 2]]),
            vec![2, 2]
        );
    }
}
