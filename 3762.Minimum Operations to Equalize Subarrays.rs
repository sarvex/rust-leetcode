#[derive(Clone, Copy, Default)]
struct SegmentNode {
    left_child: u32,
    right_child: u32,
    count: i64,
    sum: i64,
}

impl Solution {
    /// Minimum Operations to Equalize Subarrays using Persistent Segment Tree
    ///
    /// # Intuition
    /// Elements can be equalized with ±k operations iff they share the same remainder mod k.
    /// The optimal target minimizes total operations by selecting the median of quotients.
    /// A persistent segment tree enables O(log m) range median and deviation queries.
    ///
    /// # Approach
    /// 1. Compute remainder transitions via prefix array for O(1) validity check
    /// 2. Coordinate compress quotients to reduce value range
    /// 3. Build persistent segment tree with count and sum at each node
    /// 4. For each query, traverse two roots to find k-th element and compute deviation
    ///
    /// # Complexity
    /// - Time: O(n log m + q log m) where m is distinct quotient count
    /// - Space: O(n log m) for persistent nodes
    pub fn min_operations(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let num_elements = nums.len();

        let mut quotients = Vec::with_capacity(num_elements);
        let mut remainder_changes = Vec::with_capacity(num_elements);
        if let Some((&first, rest)) = nums.split_first() {
            let mut prev_remainder = first % k;
            let mut changes = 0i32;
            quotients.push((first / k) as i64);
            remainder_changes.push(0);
            for &value in rest {
                let remainder = value % k;
                changes += i32::from(remainder != prev_remainder);
                remainder_changes.push(changes);
                prev_remainder = remainder;
                quotients.push((value / k) as i64);
            }
        }

        let mut compressed_values: Vec<i64> = quotients.clone();
        compressed_values.sort_unstable();
        compressed_values.dedup();
        let value_range = compressed_values.len();

        let compressed_indices: Vec<u32> = quotients
            .iter()
            .map(|&value| (compressed_values.binary_search(&value).unwrap() + 1) as u32)
            .collect();

        let value_range_u32 = value_range as u32;
        let tree_height = (u32::BITS - value_range_u32.max(1).leading_zeros()) as usize + 1;
        let max_nodes = (num_elements + 1) * (tree_height + 1) + 10;

        let mut nodes: Vec<SegmentNode> = vec![SegmentNode::default(); max_nodes];
        let mut node_count: u32 = 1;

        #[inline]
        fn insert_value(
            previous_root: u32,
            value_range: u32,
            position: u32,
            value: i64,
            nodes: &mut [SegmentNode],
            node_count: &mut u32,
        ) -> u32 {
            let mut range_left = 1u32;
            let mut range_right = value_range;

            let mut traversal_path = [(0u32, 0u32, false); 32];
            let mut depth = 0;
            let mut previous_node = previous_root;

            while range_left < range_right {
                let current_node = *node_count;
                *node_count += 1;

                nodes[current_node as usize] = nodes[previous_node as usize];
                nodes[current_node as usize].count += 1;
                nodes[current_node as usize].sum += value;

                let mid = (range_left + range_right) >> 1;
                let goes_left = position <= mid;

                traversal_path[depth] = (previous_node, current_node, goes_left);
                depth += 1;

                if goes_left {
                    previous_node = nodes[previous_node as usize].left_child;
                    range_right = mid;
                } else {
                    previous_node = nodes[previous_node as usize].right_child;
                    range_left = mid + 1;
                }
            }

            let leaf_node = *node_count;
            *node_count += 1;
            nodes[leaf_node as usize] = nodes[previous_node as usize];
            nodes[leaf_node as usize].count += 1;
            nodes[leaf_node as usize].sum += value;

            let mut child_node = leaf_node;
            for i in (0..depth).rev() {
                let (_, current_node, went_left) = traversal_path[i];
                if went_left {
                    nodes[current_node as usize].left_child = child_node;
                } else {
                    nodes[current_node as usize].right_child = child_node;
                }
                child_node = current_node;
            }

            child_node
        }

        let mut version_roots = vec![0u32; num_elements + 1];
        for i in 0..num_elements {
            version_roots[i + 1] = insert_value(
                version_roots[i],
                value_range_u32,
                compressed_indices[i],
                quotients[i],
                &mut nodes,
                &mut node_count,
            );
        }

        #[inline]
        fn find_kth_with_prefix_stats(
            mut left_version: u32,
            mut right_version: u32,
            value_range: u32,
            mut rank: i64,
            nodes: &[SegmentNode],
        ) -> (u32, i64, i64) {
            let mut range_left = 1u32;
            let mut range_right = value_range;
            let mut prefix_count = 0i64;
            let mut prefix_sum = 0i64;

            while range_left < range_right {
                let left_node = &nodes[left_version as usize];
                let right_node = &nodes[right_version as usize];

                let left_left = left_node.left_child;
                let right_left = right_node.left_child;
                let left_subtree_count =
                    nodes[right_left as usize].count - nodes[left_left as usize].count;
                let mid = (range_left + range_right) >> 1;

                if rank <= left_subtree_count {
                    left_version = left_left;
                    right_version = right_left;
                    range_right = mid;
                } else {
                    let left_subtree_sum =
                        nodes[right_left as usize].sum - nodes[left_left as usize].sum;
                    prefix_count += left_subtree_count;
                    prefix_sum += left_subtree_sum;
                    rank -= left_subtree_count;
                    left_version = left_node.right_child;
                    right_version = right_node.right_child;
                    range_left = mid + 1;
                }
            }

            let leaf_count =
                nodes[right_version as usize].count - nodes[left_version as usize].count;
            let leaf_sum = nodes[right_version as usize].sum - nodes[left_version as usize].sum;

            prefix_count += leaf_count;
            prefix_sum += leaf_sum;

            (range_left, prefix_count, prefix_sum)
        }

        let mut results = Vec::with_capacity(queries.len());

        for query in &queries {
            let left_bound = query[0] as usize;
            let right_bound = query[1] as usize;

            if left_bound == right_bound {
                results.push(0);
                continue;
            }

            if remainder_changes[right_bound] - remainder_changes[left_bound] != 0 {
                results.push(-1);
                continue;
            }

            let subarray_length = (right_bound - left_bound + 1) as i64;
            let left_root = version_roots[left_bound];
            let right_root = version_roots[right_bound + 1];

            let median_rank = (subarray_length + 1) >> 1;
            let (median_index, count_up_to_median, sum_up_to_median) = find_kth_with_prefix_stats(
                left_root,
                right_root,
                value_range_u32,
                median_rank,
                &nodes,
            );
            let median_value = compressed_values[(median_index - 1) as usize];

            let total_sum = nodes[right_root as usize].sum - nodes[left_root as usize].sum;
            let count_above_median = subarray_length - count_up_to_median;
            let sum_above_median = total_sum - sum_up_to_median;

            let operations = median_value * count_up_to_median - sum_up_to_median
                + sum_above_median
                - median_value * count_above_median;
            results.push(operations);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_element_subarray() {
        let nums = vec![3, 6, 9];
        let queries = vec![vec![0, 0], vec![1, 1], vec![2, 2]];
        assert_eq!(Solution::min_operations(nums, 3, queries), vec![0, 0, 0]);
    }

    #[test]
    fn test_same_remainder_subarray() {
        let nums = vec![1, 4, 7];
        let queries = vec![vec![0, 2]];
        assert_eq!(Solution::min_operations(nums, 3, queries), vec![2]);
    }

    #[test]
    fn test_different_remainders_returns_negative_one() {
        let nums = vec![1, 2, 4];
        let queries = vec![vec![0, 1]];
        assert_eq!(Solution::min_operations(nums, 3, queries), vec![-1]);
    }

    #[test]
    fn test_all_equal_elements() {
        let nums = vec![5, 5, 5];
        let queries = vec![vec![0, 2]];
        assert_eq!(Solution::min_operations(nums, 1, queries), vec![0]);
    }

    #[test]
    fn test_two_element_subarray() {
        let nums = vec![2, 8];
        let queries = vec![vec![0, 1]];
        assert_eq!(Solution::min_operations(nums, 3, queries), vec![2]);
    }
}
