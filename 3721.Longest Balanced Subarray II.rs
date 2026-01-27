const MAXV: usize = 100_000;

struct SegTree {
    size: usize,
    min_val: Vec<i32>,
    max_val: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegTree {
    fn new(size: usize) -> Self {
        let tree_size = 4 * size + 5;
        Self {
            size,
            min_val: vec![0; tree_size],
            max_val: vec![0; tree_size],
            lazy: vec![0; tree_size],
        }
    }

    fn build(&mut self, array: &[i32]) {
        self.build_rec(1, 0, self.size - 1, array);
    }

    fn build_rec(&mut self, node: usize, left: usize, right: usize, array: &[i32]) {
        if left == right {
            let value = array[left];
            self.min_val[node] = value;
            self.max_val[node] = value;
            return;
        }
        let mid = (left + right) >> 1;
        let left_child = node << 1;
        let right_child = left_child | 1;
        self.build_rec(left_child, left, mid, array);
        self.build_rec(right_child, mid + 1, right, array);
        self.min_val[node] = self.min_val[left_child].min(self.min_val[right_child]);
        self.max_val[node] = self.max_val[left_child].max(self.max_val[right_child]);
    }

    fn apply(&mut self, node: usize, delta: i32) {
        self.min_val[node] += delta;
        self.max_val[node] += delta;
        self.lazy[node] += delta;
    }

    fn push_down(&mut self, node: usize) {
        let delta = self.lazy[node];
        if delta != 0 {
            let left_child = node << 1;
            let right_child = left_child | 1;
            self.apply(left_child, delta);
            self.apply(right_child, delta);
            self.lazy[node] = 0;
        }
    }

    fn pull_up(&mut self, node: usize) {
        let left_child = node << 1;
        let right_child = left_child | 1;
        self.min_val[node] = self.min_val[left_child].min(self.min_val[right_child]);
        self.max_val[node] = self.max_val[left_child].max(self.max_val[right_child]);
    }

    fn range_add(&mut self, query_left: usize, query_right: usize, delta: i32) {
        if query_left > query_right {
            return;
        }
        self.range_add_rec(1, 0, self.size - 1, query_left, query_right, delta);
    }

    fn range_add_rec(
        &mut self,
        node: usize,
        tree_left: usize,
        tree_right: usize,
        query_left: usize,
        query_right: usize,
        delta: i32,
    ) {
        if query_left <= tree_left && tree_right <= query_right {
            self.apply(node, delta);
            return;
        }
        self.push_down(node);
        let tree_mid = (tree_left + tree_right) >> 1;

        if query_left <= tree_mid {
            self.range_add_rec(
                node << 1,
                tree_left,
                tree_mid,
                query_left,
                query_right.min(tree_mid),
                delta,
            );
        }
        if query_right > tree_mid {
            self.range_add_rec(
                node << 1 | 1,
                tree_mid + 1,
                tree_right,
                query_left.max(tree_mid + 1),
                query_right,
                delta,
            );
        }
        self.pull_up(node);
    }

    fn find_rightmost_zero(&mut self, query_left: usize) -> Option<usize> {
        self.find_rightmost_zero_rec(1, 0, self.size - 1, query_left)
    }

    fn find_rightmost_zero_rec(
        &mut self,
        node: usize,
        tree_left: usize,
        tree_right: usize,
        query_left: usize,
    ) -> Option<usize> {
        if tree_right < query_left || self.min_val[node] > 0 || self.max_val[node] < 0 {
            return None;
        }
        if tree_left == tree_right {
            return Some(tree_left);
        }

        self.push_down(node);
        let tree_mid = (tree_left + tree_right) >> 1;

        self.find_rightmost_zero_rec(node << 1 | 1, tree_mid + 1, tree_right, query_left)
            .or_else(|| self.find_rightmost_zero_rec(node << 1, tree_left, tree_mid, query_left))
    }
}

impl Solution {
    /// Longest Balanced Subarray II
    ///
    /// # Intuition
    /// A subarray is balanced when distinct evens equals distinct odds. Use segment tree
    /// with lazy propagation to efficiently track balance differences and find rightmost
    /// balanced positions as we slide the left pointer.
    ///
    /// # Approach
    /// Build difference array (odd_count - even_count) for position 0. Use segment tree to:
    /// 1. Find rightmost position where difference equals 0 (balanced subarray)
    /// 2. Update ranges when element becomes duplicate as we slide left pointer
    /// Segment tree supports O(log n) range updates and queries for finding zeros.
    ///
    /// # Complexity
    /// - Time: O(n log n) - segment tree operations
    /// - Space: O(n) - segment tree and auxiliary arrays
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let num_elements = nums.len();
        if num_elements < 2 {
            return 0;
        }

        let num_elements_u32 = num_elements as u32;

        // Precompute next occurrence of each value
        let mut next_occurrence: Vec<u32> = vec![num_elements_u32; num_elements];
        let mut last_position: Vec<u32> = vec![num_elements_u32; MAXV + 1];

        for index in (0..num_elements).rev() {
            let value = nums[index] as usize;
            next_occurrence[index] = last_position[value];
            last_position[value] = index as u32;
        }

        // Build difference array: diff[r] = (#odd - #even) for subarray [0, r]
        let mut seen_values: Vec<u8> = vec![0; MAXV + 1];
        let mut difference: Vec<i32> = vec![0; num_elements];
        let mut balance = 0;
        let mut distinct_odd_count = 0;
        let mut distinct_even_count = 0;

        for index in 0..num_elements {
            let value = nums[index] as usize;
            if seen_values[value] == 0 {
                seen_values[value] = 1;
                balance += if (value & 1) == 1 { 1 } else { -1 };
                if (value & 1) == 1 {
                    distinct_odd_count += 1;
                } else {
                    distinct_even_count += 1;
                }
            }
            difference[index] = balance;
        }

        // Early exits
        if distinct_odd_count == 0 || distinct_even_count == 0 {
            return 0;
        }
        if balance == 0 {
            return num_elements as i32;
        }

        let mut seg_tree = SegTree::new(num_elements);
        seg_tree.build(&difference);

        let mut max_length = 0;

        // Slide left pointer, find rightmost balanced position
        for left_pos in 0..num_elements {
            if let Some(right_pos) = seg_tree.find_rightmost_zero(left_pos) {
                max_length = max_length.max(right_pos + 1 - left_pos);
            }

            let next_pos = next_occurrence[left_pos] as usize;
            if next_pos > left_pos + 1 {
                // Update range when element at left_pos becomes duplicate
                let delta = if (nums[left_pos] & 1) != 0 { -1 } else { 1 };
                seg_tree.range_add(left_pos + 1, next_pos - 1, delta);
            }
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::longest_balanced(vec![1]), 0);
        assert_eq!(Solution::longest_balanced(vec![2]), 0);
    }

    #[test]
    fn test_two_elements_balanced() {
        assert_eq!(Solution::longest_balanced(vec![1, 2]), 2);
        assert_eq!(Solution::longest_balanced(vec![2, 1]), 2);
    }

    #[test]
    fn test_all_same_parity() {
        assert_eq!(Solution::longest_balanced(vec![1, 3, 5, 7]), 0);
        assert_eq!(Solution::longest_balanced(vec![2, 4, 6, 8]), 0);
    }

    #[test]
    fn test_with_duplicates() {
        assert_eq!(Solution::longest_balanced(vec![1, 1, 2, 2]), 4);
        assert_eq!(Solution::longest_balanced(vec![1, 2, 1, 2]), 4);
    }
}
