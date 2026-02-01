struct AlternatingGroups {
    n: usize,
    colors: Vec<u8>,
    fenwick: Fenwick,
    boundary_set: BoundarySet,
}

impl AlternatingGroups {
    fn new(colors: Vec<i32>) -> Self {
        let n = colors.len();
        let mut fenwick = Fenwick::new(n);
        let mut boundary_set = BoundarySet::new(n);
        let colors: Vec<u8> = colors.into_iter().map(|c| c as u8).collect();

        let mut start = None;
        for i in 0..n {
            let prev = if i == 0 { n - 1 } else { i - 1 };
            if colors[prev] == colors[i] {
                start = Some(i);
                break;
            }
        }

        if let Some(start) = start {
            let mut i = start;
            loop {
                boundary_set.insert(i);
                let mut j = (i + 1) % n;
                while colors[j] != colors[(j + n - 1) % n] {
                    j = (j + 1) % n;
                }
                let len = (j + n - i) % n;
                let interval_len = if len == 0 { n } else { len };
                fenwick.add(interval_len, 1);
                i = j;
                if i == start {
                    break;
                }
            }
        } else {
            // Fully alternating even cycle.
            boundary_set.insert(0);
            fenwick.add(n, 1);
        }

        Self {
            n,
            colors,
            fenwick,
            boundary_set,
        }
    }

    fn query(&self, size: usize) -> i64 {
        if self.n % 2 == 0 && self.boundary_set.len() <= 1 {
            return self.n as i64;
        }
        let all = self.fenwick.prefix(self.n);
        let small = self.fenwick.prefix(size - 1);
        let count_ge = all.0 - small.0;
        let sum_ge = all.1 - small.1;
        sum_ge - (size as i64 - 1) * count_ge
    }

    fn toggle(&mut self, index: usize, color: u8) {
        if self.colors[index] == color {
            return;
        }

        self.colors[index] = color;
        let n = self.n;
        let (start, end) = self.find_containing_interval(index);
        let interval_len = (end + n - start) % n;

        if start == end {
            // A single interval of size n (odd cycle) or a fully alternating even cycle.
            if n % 2 == 1 {
                if (index + 1) % n == start {
                    self.boundary_set.clear();
                    self.boundary_set.insert(index);
                } else if index == start {
                    self.boundary_set.clear();
                    self.boundary_set.insert((index + 1) % n);
                } else {
                    self.boundary_set.insert(index);
                    self.boundary_set.insert((index + 1) % n);
                    let left_len = (index + n - start) % n;
                    let right_len = (end + n - (index + 1)) % n;
                    self.fenwick.add(n, -1);
                    self.fenwick.add(left_len, 1);
                    self.fenwick.add(right_len, 1);
                }
            } else {
                self.fenwick.add(n, -1);
                self.fenwick.add(n - 1, 1);
                self.fenwick.add(1, 1);
                self.boundary_set.clear();
                self.boundary_set.insert(index);
                self.boundary_set.insert((index + 1) % n);
            }
        } else if interval_len == 1 {
            // Singleton interval: connect both adjacent intervals.
            self.boundary_set.remove(start);
            self.boundary_set.remove(end);
            if self.boundary_set.is_empty() {
                self.boundary_set.insert(0);
                self.fenwick.add(1, -1);
                self.fenwick.add(n - 1, -1);
                self.fenwick.add(n, 1);
                return;
            }
            let (new_start, new_end) = self.find_containing_interval(index);
            let left_len = (index + n - new_start) % n;
            let right_len = (new_end + n - (index + 1)) % n;
            let mut merged_len = (new_end + n - new_start) % n;
            if merged_len == 0 {
                merged_len = n;
            }
            self.fenwick.add(1, -1);
            self.fenwick.add(left_len, -1);
            self.fenwick.add(right_len, -1);
            self.fenwick.add(merged_len, 1);
        } else if index == start {
            // At the left boundary: shift boundary right by one.
            let (left_start, left_end) = self.find_containing_interval((index + n - 1) % n);
            let left_len = (left_end + n - left_start) % n;
            debug_assert_eq!(left_end, start);
            self.fenwick.add(interval_len, -1);
            self.fenwick.add(left_len, -1);
            self.fenwick.add(interval_len - 1, 1);
            self.fenwick.add(left_len + 1, 1);
            self.boundary_set.remove(index);
            self.boundary_set.insert((index + 1) % n);
        } else if (index + 1) % n == end {
            // At the right boundary: shift boundary left by one.
            let (right_start, right_end) = self.find_containing_interval((index + 1) % n);
            let right_len = (right_end + n - right_start) % n;
            debug_assert_eq!(right_start, end);
            self.fenwick.add(interval_len, -1);
            self.fenwick.add(right_len, -1);
            self.fenwick.add(interval_len - 1, 1);
            self.fenwick.add(right_len + 1, 1);
            self.boundary_set.remove((index + 1) % n);
            self.boundary_set.insert(index);
        } else {
            // Split the interval into two parts plus a singleton.
            let left_len = (index + n - start) % n;
            let right_len = (end + n - (index + 1)) % n;
            self.fenwick.add(interval_len, -1);
            self.fenwick.add(1, 1);
            self.fenwick.add(left_len, 1);
            self.fenwick.add(right_len, 1);
            self.boundary_set.insert(index);
            self.boundary_set.insert((index + 1) % n);
        }
    }

    fn find_containing_interval(&self, index: usize) -> (usize, usize) {
        let start = self.boundary_set.prev(index);
        let end = self.boundary_set.next(index);
        (start, end)
    }
}

impl Solution {
    /// Interval-based counting with a Fenwick tree over alternating runs.
    ///
    /// # Intuition
    /// A size-`k` alternating group is valid when the `k-1` adjacent edges inside it alternate.
    /// These windows live inside maximal alternating intervals between equal-adjacent boundaries.
    ///
    /// # Approach
    /// - Track boundary indices `i` where `colors[i - 1] == colors[i]` using a `BTreeSet`.
    /// - Each interval between consecutive boundaries has length `L` and contributes
    ///   `max(0, L - (k - 1))` groups for a query size `k`.
    /// - Maintain a Fenwick tree keyed by `L` to query counts and total lengths in `O(log n)`.
    /// - A color update changes at most two boundaries, which merges or splits `O(1)` intervals.
    ///
    /// # Complexity
    /// - Time: O((n + q) log n)
    /// - Space: O(n)
    pub fn number_of_alternating_groups(
        colors: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut state = AlternatingGroups::new(colors);
        let mut results = Vec::new();
        for query in queries {
            match query[0] {
                1 => {
                    let size = query[1] as usize;
                    results.push(state.query(size) as i32);
                }
                2 => {
                    let index = query[1] as usize;
                    let color = query[2] as u8;
                    state.toggle(index, color);
                }
                _ => {}
            }
        }

        results
    }
}

struct Fenwick {
    tree: Vec<(i64, i64)>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![(0, 0); n + 1],
        }
    }

    fn size(&self) -> usize {
        self.tree.len() - 1
    }

    fn add(&mut self, index: usize, value: i64) {
        if index == 0 {
            return;
        }
        let mut i = index;
        while i < self.tree.len() {
            self.tree[i].0 += value;
            self.tree[i].1 += value * index as i64;
            i += Self::lsb(i);
        }
    }

    fn prefix(&self, index: usize) -> (i64, i64) {
        let mut sum0 = 0_i64;
        let mut sum1 = 0_i64;
        let mut i = index;
        while i > 0 {
            sum0 += self.tree[i].0;
            sum1 += self.tree[i].1;
            i -= Self::lsb(i);
        }
        (sum0, sum1)
    }

    fn lsb(i: usize) -> usize {
        i & i.wrapping_neg()
    }
}

struct BoundarySet {
    n: usize,
    present: Vec<bool>,
    bit: FenwickCount,
    count: usize,
}

impl BoundarySet {
    fn new(n: usize) -> Self {
        Self {
            n,
            present: vec![false; n],
            bit: FenwickCount::new(n),
            count: 0,
        }
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn insert(&mut self, index: usize) {
        if !self.present[index] {
            self.present[index] = true;
            self.count += 1;
            self.bit.add(index + 1, 1);
        }
    }

    fn remove(&mut self, index: usize) {
        if self.present[index] {
            self.present[index] = false;
            self.count -= 1;
            self.bit.add(index + 1, -1);
        }
    }

    fn clear(&mut self) {
        while self.count > 0 {
            let index = self.kth(1);
            self.remove(index);
        }
    }

    fn prev(&self, index: usize) -> usize {
        let prefix = self.bit.sum(index + 1) as usize;
        if prefix > 0 {
            self.kth(prefix)
        } else {
            self.kth(self.count)
        }
    }

    fn next(&self, index: usize) -> usize {
        let prefix = self.bit.sum(index + 1) as usize;
        let order = if prefix < self.count { prefix + 1 } else { 1 };
        self.kth(order)
    }

    fn kth(&self, order: usize) -> usize {
        self.bit.kth(order as i32) - 1
    }
}

struct FenwickCount {
    tree: Vec<i32>,
    max_bit: usize,
}

impl FenwickCount {
    fn new(n: usize) -> Self {
        let mut max_bit = 1;
        while max_bit < n + 1 {
            max_bit <<= 1;
        }
        Self {
            tree: vec![0; n + 1],
            max_bit: max_bit >> 1,
        }
    }

    fn add(&mut self, index: usize, value: i32) {
        let mut i = index;
        while i < self.tree.len() {
            self.tree[i] += value;
            i += Self::lsb(i);
        }
    }

    fn sum(&self, index: usize) -> i32 {
        let mut res = 0_i32;
        let mut i = index;
        while i > 0 {
            res += self.tree[i];
            i -= Self::lsb(i);
        }
        res
    }

    fn kth(&self, mut k: i32) -> usize {
        let mut idx = 0_usize;
        let mut bit = self.max_bit;
        while bit > 0 {
            let next = idx + bit;
            if next < self.tree.len() && self.tree[next] < k {
                k -= self.tree[next];
                idx = next;
            }
            bit >>= 1;
        }
        idx + 1
    }

    fn lsb(i: usize) -> usize {
        i & i.wrapping_neg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let colors = vec![0, 1, 1, 0, 1];
        let queries = vec![vec![2, 1, 0], vec![1, 4]];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), vec![2]);
    }

    #[test]
    fn example_two() {
        let colors = vec![0, 0, 1, 0, 1, 1];
        let queries = vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), vec![2, 0]);
    }

    #[test]
    fn transition_to_all_ones() {
        let colors = vec![0, 0, 1, 0];
        let queries = vec![vec![1, 3], vec![2, 0, 1], vec![1, 3]];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), vec![1, 4]);
    }

    #[test]
    fn all_ones_then_break() {
        let colors = vec![0, 1, 0, 1];
        let queries = vec![vec![1, 3], vec![2, 1, 0], vec![1, 3]];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), vec![4, 1]);
    }
}
