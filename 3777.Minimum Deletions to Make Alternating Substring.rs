pub struct Fenwick {
    nums: Vec<i32>,
    sums: Vec<i32>,
}

const fn lowbit(i: usize) -> usize {
    let i = i as isize;
    (i & -i) as usize
}

impl Fenwick {
    pub fn from_slice(nums: &[i32]) -> Self {
        let len = nums.len() + 1;
        let nums = nums.to_vec();
        let mut sums = vec![0; len];
        for i in 1..len {
            sums[i] += nums[i - 1];
            let j = i + lowbit(i);
            if j < len {
                sums[j] += sums[i];
            }
        }
        Self { nums, sums }
    }

    pub fn update(&mut self, index: usize, val: i32) {
        let delta = val - self.nums[index];
        self.nums[index] = val;
        let mut i = index + 1;
        let len = self.sums.len();
        while i < len {
            self.sums[i] += delta;
            i += lowbit(i);
        }
    }

    fn query(&self, mut index: usize) -> i32 {
        index += 1;
        let mut val = 0;
        while index > 0 {
            val += self.sums[index];
            index -= lowbit(index);
        }
        val
    }

    pub fn query_range(&self, left: usize, right: usize) -> i32 {
        self.query(right) - if left > 0 { self.query(left - 1) } else { 0 }
    }
}

impl Solution {
    /// Fenwick Tree for Alternating Substring Queries
    ///
    /// # Intuition
    /// Count adjacent equal pairs in the string.
    /// Each equal pair requires one deletion to make alternating.
    ///
    /// # Approach
    /// Use Fenwick tree with O(n) construction via prefix propagation.
    /// Track current values to compute deltas for updates.
    /// Range query gives count of equal pairs in substring.
    ///
    /// # Complexity
    /// - Time: O(n + q log n)
    /// - Space: O(n)
    pub fn min_deletions(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len = s.len();
        let mut bytes = s.as_bytes().to_vec();
        let mut values = vec![0];
        for win in bytes.windows(2) {
            values.push(i32::from(win[0] == win[1]));
        }
        let mut tree = Fenwick::from_slice(&values);
        let mut ans = vec![];
        for query in &queries {
            if query[0] == 1 {
                let idx = query[1] as usize;
                bytes[idx] = if bytes[idx] == b'A' { b'B' } else { b'A' };
                if idx > 0 {
                    tree.update(idx, i32::from(bytes[idx - 1] == bytes[idx]));
                }
                if idx + 1 < len {
                    tree.update(idx + 1, i32::from(bytes[idx] == bytes[idx + 1]));
                }
            } else {
                let left = query[1] as usize;
                let right = query[2] as usize;
                ans.push(tree.query_range(left + 1, right));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "ABA".to_string();
        let queries = vec![vec![2, 1, 2], vec![1, 1], vec![2, 0, 2]];
        assert_eq!(Solution::min_deletions(s, queries), vec![0, 2]);
    }

    #[test]
    fn test_example_2() {
        let s = "ABB".to_string();
        let queries = vec![vec![2, 0, 2], vec![1, 2], vec![2, 0, 2]];
        assert_eq!(Solution::min_deletions(s, queries), vec![1, 0]);
    }

    #[test]
    fn test_example_3() {
        let s = "BABA".to_string();
        let queries = vec![vec![2, 0, 3], vec![1, 1], vec![2, 1, 3]];
        assert_eq!(Solution::min_deletions(s, queries), vec![0, 1]);
    }

    #[test]
    fn test_single_char() {
        let s = "A".to_string();
        let queries = vec![vec![2, 0, 0]];
        assert_eq!(Solution::min_deletions(s, queries), vec![0]);
    }

    #[test]
    fn test_all_same() {
        let s = "AAAA".to_string();
        let queries = vec![vec![2, 0, 3]];
        assert_eq!(Solution::min_deletions(s, queries), vec![3]);
    }
}
