struct BinaryIndexedTree {
    size: usize,
    data: Vec<i64>,
}

impl BinaryIndexedTree {
    fn new(array_size: usize) -> Self {
        let size = array_size + 1;
        Self {
            size,
            data: vec![0; size + 1],
        }
    }

    fn update(&mut self, index: usize, delta: i64) {
        let mut idx = (index + 1) as i32;
        while (idx as usize) <= self.size {
            self.data[idx as usize] += delta;
            idx += idx & -idx;
        }
    }

    fn prefix_sum(&self, index: usize) -> i64 {
        let mut idx = (index + 1) as i32;
        let mut total = 0i64;
        while idx > 0 {
            total += self.data[idx as usize];
            idx -= idx & -idx;
        }
        total
    }

    fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.prefix_sum(right) - self.prefix_sum(left - 1)
    }
}

struct PopcountDepthCache {
    depth_cache: Vec<i32>,
}

impl PopcountDepthCache {
    fn new() -> Self {
        let mut depth_cache = Vec::with_capacity(65);
        for value in 0..=64 {
            depth_cache.push(Self::compute_depth(value as i32) - 1);
        }
        Self { depth_cache }
    }

    fn get_depth(&self, number: i64) -> i32 {
        let number = number as usize;
        if number < self.depth_cache.len() {
            self.depth_cache[number]
        } else {
            let popcount = number.count_ones() as usize;
            self.depth_cache[popcount] + 1
        }
    }

    fn compute_depth(value: i32) -> i32 {
        if value <= 1 {
            value
        } else {
            1 + Self::compute_depth(value.count_ones() as i32)
        }
    }
}

impl Solution {
    /// Fenwick Trees for Popcount-Depth Range Queries
    ///
    /// # Intuition
    /// Track elements by their popcount-depth using separate Fenwick trees.
    /// Each tree counts elements with a specific depth in O(log n) per query.
    ///
    /// # Approach
    /// 1. Precompute depth cache for numbers 0-64 (max popcount for i64)
    /// 2. For larger numbers, compute one popcount then lookup cached depth
    /// 3. Maintain 5 Fenwick trees (one per depth 0-4, max depth is 4)
    /// 4. For updates: skip if depth unchanged, otherwise update both trees
    /// 5. For queries: query the tree for target depth k
    ///
    /// # Complexity
    /// - Time: O(n log n + q log n) where n is array size, q is query count
    /// - Space: O(n) for Fenwick trees
    pub fn popcount_depth(mut nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
        let array_size = nums.len();
        let depth_cache = PopcountDepthCache::new();
        let mut depth_trees = (0..5).map(|_| BinaryIndexedTree::new(array_size)).collect::<Vec<_>>();

        for (index, &number) in nums.iter().enumerate() {
            let depth = depth_cache.get_depth(number) as usize;
            if depth < depth_trees.len() {
                depth_trees[depth].update(index, 1);
            }
        }

        let mut result = Vec::new();
        for query in queries {
            if query[0] == 1 {
                let left = query[1] as usize;
                let right = query[2] as usize;
                let target_depth = query[3] as usize;
                if target_depth < depth_trees.len() {
                    result.push(depth_trees[target_depth].range_sum(left, right) as i32);
                } else {
                    result.push(0);
                }
            } else {
                let index = query[1] as usize;
                let new_value = query[2];
                let old_depth = depth_cache.get_depth(nums[index]) as usize;
                let new_depth = depth_cache.get_depth(new_value) as usize;

                nums[index] = new_value;
                if old_depth != new_depth {
                    if old_depth < depth_trees.len() {
                        depth_trees[old_depth].update(index, -1);
                    }
                    if new_depth < depth_trees.len() {
                        depth_trees[new_depth].update(index, 1);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![2, 4];
        let queries = vec![vec![1, 0, 1, 1], vec![2, 1, 1], vec![1, 0, 1, 0]];
        assert_eq!(Solution::popcount_depth(nums, queries), vec![2, 1]);
    }

    #[test]
    fn test_example2() {
        let nums = vec![3, 5, 6];
        let queries = vec![
            vec![1, 0, 2, 2],
            vec![2, 1, 4],
            vec![1, 1, 2, 1],
            vec![1, 0, 1, 0],
        ];
        assert_eq!(Solution::popcount_depth(nums, queries), vec![3, 1, 0]);
    }

    #[test]
    fn test_example3() {
        let nums = vec![1, 2];
        let queries = vec![
            vec![1, 0, 1, 1],
            vec![2, 0, 3],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 2],
        ];
        assert_eq!(Solution::popcount_depth(nums, queries), vec![1, 0, 1]);
    }
}
