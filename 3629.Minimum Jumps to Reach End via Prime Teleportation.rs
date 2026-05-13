use std::sync::LazyLock;

/// Precomputed sieve: `SIEVE[v] = (smallest_prime_factor_of_v, v / p^k)` where
/// `p^k` is the full power of the smallest prime factor dividing `v`.
/// This lets us fully factorise any value ≤ 10^6 in O(log v) chain lookups
/// instead of O(√v) trial division.
///
/// `SIEVE[v] = (-1, -1)` means v is 0 or 1 (no prime factors).
/// `SIEVE[v] = (v, 1)` means v is prime.
static SIEVE: LazyLock<Vec<(i32, i32)>> = LazyLock::new(|| {
    let n: i32 = 1_000_000;
    let sqrt_n: i32 = 1_000;
    let mut sieve = vec![(-1_i32, -1_i32); (n + 1) as usize];
    // For each prime p found by the sieve, mark all multiples j of p.
    // Store (p, j/p^k) so factorisation is a simple chain: follow .1 until 1.
    for p in 2..=sqrt_n {
        if sieve[p as usize].0 == -1 {
            sieve[p as usize] = (p, 1);
            let mut j = p * p;
            let mut cofactor_base = p; // j / p at each step
            while j <= n {
                if sieve[j as usize].0 == -1 {
                    // Remove all factors of p from j to get the cofactor.
                    let mut x = cofactor_base;
                    while x % p == 0 {
                        x /= p;
                    }
                    sieve[j as usize] = (p, x);
                }
                j += p;
                cofactor_base += 1;
            }
        }
    }
    // Values > sqrt_n that were never marked are prime.
    for v in (sqrt_n + 1)..=n {
        if sieve[v as usize].0 == -1 {
            sieve[v as usize] = (v, 1);
        }
    }
    sieve
});

impl Solution {
    /// Level-by-level BFS with a precomputed SPF sieve and virtual prime nodes.
    ///
    /// # Intuition
    /// Teleportation from index `i` (when `nums[i]` is prime `p`) to any `j`
    /// where `p | nums[j]` would create O(n²) edges naively. Virtual nodes
    /// collapse each prime's fan-out: one edge from source to virtual(p), then
    /// one edge from virtual(p) to each destination — all within the same BFS
    /// level (cost 1 total). Only primes that actually appear as `nums[i]` values
    /// need virtual nodes, keeping the graph small.
    ///
    /// Factorisation uses a precomputed smallest-prime-factor sieve so each
    /// value is factorised in O(log val) instead of O(√val).
    ///
    /// # Approach
    /// 1. Sieve (static, computed once): `SIEVE[v] = (spf, cofactor)` enables
    ///    O(log v) factorisation by repeatedly following the cofactor chain.
    /// 2. Assign an id to each prime that appears as a `nums[i]` value (sources).
    /// 3. Build adjacency lists: for each index `j`, walk its prime factorisation
    ///    and append `j` to the list of every prime id that divides `nums[j]`.
    /// 4. BFS with two alternating `Vec`s (current level / next level):
    ///    - Expand adjacent indices and all reachable indices via unvisited primes.
    ///    - Return as soon as index `n-1` is reached.
    ///
    /// # Complexity
    /// - Time: O(n · log(max_val)) amortised (sieve is O(max_val · log log max_val)
    ///   but paid once); BFS visits each node at most once.
    /// - Space: O(max_val) for the sieve + O(n + P) for BFS state
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let last = (n - 1) as i32;

        if n <= 2 {
            return last;
        }

        let sieve: &[(i32, i32)] = &SIEVE;

        // Fast path: can we teleport directly from index 0 to index n-1?
        {
            let a = nums[0];
            let b = nums[n - 1];
            if sieve[a as usize].0 == a && b % a == 0 {
                return 1;
            }
        }

        if n == 3 {
            return 2;
        }

        // Fast path: all values are distinct increasing primes → must walk every step.
        {
            let mut prev = 0_i32;
            let all_distinct_primes = nums.iter().all(|&x| {
                let ok = prev < x && sieve[x as usize].0 == x;
                prev = x;
                ok
            });
            if all_distinct_primes {
                return last;
            }
        }

        let max_val = *nums.iter().max().unwrap() as usize;

        // All 1s: no prime factors, no teleportation.
        if max_val == 1 {
            return last;
        }

        // Assign ids to primes that appear as nums[i] values (teleport sources).
        // prime_id[p] = id of virtual node for prime p, or -1 if not a source.
        let mut prime_id = vec![-1_i32; max_val + 1];
        let mut num_primes = 0_i32;
        for &x in &nums {
            let xu = x as usize;
            if sieve[xu].0 == x && prime_id[xu] == -1 {
                prime_id[xu] = num_primes;
                num_primes += 1;
            }
        }

        // Build adjacency: al[prime_id] = list of indices j where prime | nums[j].
        let mut al: Vec<Vec<i32>> = vec![Vec::new(); num_primes as usize];
        for (j, &x) in nums.iter().enumerate() {
            let mut y = x;
            while y > 1 {
                let (p, cofactor) = sieve[y as usize];
                let id = prime_id[p as usize];
                if id != -1 {
                    al[id as usize].push(j as i32);
                }
                y = cofactor;
            }
        }

        // Level-by-level BFS with two alternating vecs.
        let mut visited = vec![false; n];
        let mut visited_prime = vec![false; num_primes as usize];

        let mut enqueue = |q: &mut Vec<i32>, u: i32| {
            let uu = u as usize;
            if !visited[uu] {
                visited[uu] = true;
                q.push(u);
            }
        };

        let mut current = Vec::with_capacity(n / 2);
        enqueue(&mut current, 0);
        let mut next = Vec::with_capacity(n / 2);

        for dist in 1_i32.. {
            for u in current.drain(..) {
                // Adjacent step left.
                if u > 0 {
                    enqueue(&mut next, u - 1);
                }
                // Adjacent step right.
                if u < last {
                    if u + 1 == last {
                        return dist;
                    }
                    enqueue(&mut next, u + 1);
                }
                // Teleport: nums[u] must be prime and have a virtual node.
                let id = prime_id[nums[u as usize] as usize];
                if id != -1 {
                    let id = id as usize;
                    if !visited_prime[id] {
                        visited_prime[id] = true;
                        for &v in &al[id] {
                            if v == last {
                                return dist;
                            }
                            enqueue(&mut next, v);
                        }
                    }
                }
            }
            std::mem::swap(&mut current, &mut next);
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 0 →(adj)→ 1 →(teleport via 2)→ 3
        assert_eq!(Solution::min_jumps(vec![1, 2, 4, 6]), 2);
    }

    #[test]
    fn test_example_2() {
        // 0 →(adj)→ 1 →(teleport via 3)→ 4
        assert_eq!(Solution::min_jumps(vec![2, 3, 4, 7, 9]), 2);
    }

    #[test]
    fn test_example_3() {
        // No teleportation possible: 0 → 1 → 2 → 3
        assert_eq!(Solution::min_jumps(vec![4, 6, 5, 8]), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_jumps(vec![7]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::min_jumps(vec![1, 1]), 1);
        assert_eq!(Solution::min_jumps(vec![2, 4]), 1);
    }

    #[test]
    fn test_all_ones_no_teleport() {
        // 1 has no prime factors; must walk every step.
        assert_eq!(Solution::min_jumps(vec![1, 1, 1, 1, 1]), 4);
    }

    #[test]
    fn test_teleport_skips_many() {
        // nums[0]=2 (prime), nums[99]=2 → direct teleport in 1 jump.
        let mut nums = vec![1_i32; 100];
        nums[0] = 2;
        nums[99] = 2;
        assert_eq!(Solution::min_jumps(nums), 1);
    }

    #[test]
    fn test_large_prime_value() {
        // nums[0]=999983 (prime), nums[2]=999983 → direct teleport in 1 jump.
        assert_eq!(Solution::min_jumps(vec![999983, 1, 999983]), 1);
    }

    #[test]
    fn test_source_not_prime_no_teleport() {
        // nums[0]=4 (not prime) → no teleport from index 0; must walk.
        assert_eq!(Solution::min_jumps(vec![4, 1, 4]), 2);
    }

    #[test]
    fn test_three_elements() {
        // n=3 always returns 2 (fast path).
        assert_eq!(Solution::min_jumps(vec![1, 1, 1]), 2);
    }
}
