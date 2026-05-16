struct Solution;

impl Solution {
    /// Union-Find with monotonic stack to find max reachable value per index.
    ///
    /// # Intuition
    /// Two indices i < j are directly connected (bidirectionally) iff nums[i] > nums[j]:
    /// i can jump forward to j, and j can jump backward to i under the same condition.
    /// The answer for each index is the maximum value in its connected component.
    ///
    /// # Approach
    /// Union-Find (DSU) with union-by-rank and iterative path compression, where each
    /// root also tracks the component's maximum value.
    ///
    /// Sweep left to right with a monotonic decreasing stack of indices:
    ///
    /// For each index j:
    /// 1. Pop all stack tops where nums[top] > nums[j] and union(top, j). These are
    ///    the direct "previous greater element" connections — top can jump forward to j.
    /// 2. After popping, check the remaining stack top. Its DSU component may contain
    ///    elements with value > nums[j] that were popped earlier and merged in. If the
    ///    component max exceeds nums[j], union j into that component — those larger
    ///    elements can reach j via forward jumps, and j can reach them via backward jumps.
    /// 3. Push j onto the stack.
    ///
    /// # Complexity
    /// - Time: O(n · α(n)) ≈ O(n)
    /// - Space: O(n)
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut parent: Vec<u32> = (0..n as u32).collect();
        let mut rank: Vec<u8> = vec![0; n];
        let mut max_val: Vec<i32> = nums.clone();

        // Iterative find with full path compression
        let find = |parent: &mut Vec<u32>, mut x: usize| -> usize {
            // Find root
            let mut root = x;
            while parent[root] as usize != root {
                root = parent[root] as usize;
            }
            // Path compression: point all nodes on the path directly to root
            while parent[x] as usize != root {
                let next = parent[x] as usize;
                parent[x] = root as u32;
                x = next;
            }
            root
        };

        // Union by rank, propagating max value to the new root
        let union = |parent: &mut Vec<u32>,
                     rank: &mut Vec<u8>,
                     max_val: &mut Vec<i32>,
                     a: usize,
                     b: usize| {
            let ra = {
                let mut x = a;
                let mut root = x;
                while parent[root] as usize != root {
                    root = parent[root] as usize;
                }
                while parent[x] as usize != root {
                    let next = parent[x] as usize;
                    parent[x] = root as u32;
                    x = next;
                }
                root
            };
            let rb = {
                let mut x = b;
                let mut root = x;
                while parent[root] as usize != root {
                    root = parent[root] as usize;
                }
                while parent[x] as usize != root {
                    let next = parent[x] as usize;
                    parent[x] = root as u32;
                    x = next;
                }
                root
            };
            if ra == rb {
                return;
            }
            let combined_max = max_val[ra].max(max_val[rb]);
            let (new_root, old_root) = if rank[ra] >= rank[rb] {
                (ra, rb)
            } else {
                (rb, ra)
            };
            parent[old_root] = new_root as u32;
            if rank[ra] == rank[rb] {
                rank[new_root] += 1;
            }
            max_val[new_root] = combined_max;
        };

        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for j in 0..n {
            // Pop all stack tops that can directly jump forward to j
            while let Some(&top) = stack.last() {
                if nums[top] > nums[j] {
                    stack.pop();
                    union(&mut parent, &mut rank, &mut max_val, top, j);
                } else {
                    break;
                }
            }

            // The remaining stack top may have a component max > nums[j] due to
            // previously merged larger elements — connect j into that component.
            if let Some(&top) = stack.last() {
                let root = find(&mut parent, top);
                if max_val[root] > nums[j] {
                    union(&mut parent, &mut rank, &mut max_val, top, j);
                }
            }

            stack.push(j);
        }

        (0..n).map(|i| max_val[find(&mut parent, i)]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // [2,1,3]: i=1 connects back to i=0 (2>1). i=2 isolated (no larger left). ans=[2,2,3]
        assert_eq!(Solution::max_value(vec![2, 1, 3]), vec![2, 2, 3]);
    }

    #[test]
    fn test_example_2() {
        // [2,3,1]: 0->2 (1<2), 2->1 (3>1); all connected, max=3
        assert_eq!(Solution::max_value(vec![2, 3, 1]), vec![3, 3, 3]);
    }

    #[test]
    fn test_reported_wrong_answer() {
        // [13,4,11]: 0->1 (4<13), 0->2 (11<13); all connected, max=13
        assert_eq!(Solution::max_value(vec![13, 4, 11]), vec![13, 13, 13]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_value(vec![42]), vec![42]);
    }

    #[test]
    fn test_strictly_increasing() {
        // No valid jumps; each index isolated.
        assert_eq!(Solution::max_value(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_strictly_decreasing() {
        // Every pair i<j has nums[j]<nums[i]; all connected, max=4.
        assert_eq!(Solution::max_value(vec![4, 3, 2, 1]), vec![4, 4, 4, 4]);
    }

    #[test]
    fn test_all_equal() {
        // No jumps possible; each index isolated.
        assert_eq!(Solution::max_value(vec![5, 5, 5]), vec![5, 5, 5]);
    }

    #[test]
    fn test_valley() {
        // [3,1,3]: 0->1 (1<3), 1->2 (3>1); all connected, max=3
        assert_eq!(Solution::max_value(vec![3, 1, 3]), vec![3, 3, 3]);
    }

    #[test]
    fn test_peak() {
        // [1,3,1]: i=0 isolated. i=1->2 (1<3). i=2->1 (3>1). ans=[1,3,3]
        assert_eq!(Solution::max_value(vec![1, 3, 1]), vec![1, 3, 3]);
    }

    #[test]
    fn test_two_elements_ascending() {
        // [1,2]: no valid jumps.
        assert_eq!(Solution::max_value(vec![1, 2]), vec![1, 2]);
    }

    #[test]
    fn test_two_elements_descending() {
        // [2,1]: 0->1 (1<2). Connected; max=2.
        assert_eq!(Solution::max_value(vec![2, 1]), vec![2, 2]);
    }

    #[test]
    fn test_longer_chain() {
        // [5,1,4,2,3]: all < 5, all connected via 0; max=5.
        assert_eq!(
            Solution::max_value(vec![5, 1, 4, 2, 3]),
            vec![5, 5, 5, 5, 5]
        );
    }

    #[test]
    fn test_two_separate_groups() {
        // [10,1,8,2]: all < 10, all connected via 0; max=10.
        assert_eq!(Solution::max_value(vec![10, 1, 8, 2]), vec![10, 10, 10, 10]);
    }

    #[test]
    fn test_large_alternating() {
        // [100,1,100,1,...]: alternating high/low; all connected, max=100.
        let n = 1000;
        let nums: Vec<i32> = (0..n).map(|i| if i % 2 == 0 { 100 } else { 1 }).collect();
        let expected: Vec<i32> = vec![100; n];
        assert_eq!(Solution::max_value(nums), expected);
    }

    #[test]
    fn test_large_decreasing() {
        // Strictly decreasing large input; all connected, max = n.
        let n = 1000;
        let nums: Vec<i32> = (0..n as i32).rev().map(|x| x + 1).collect();
        let expected: Vec<i32> = vec![n as i32; n];
        assert_eq!(Solution::max_value(nums), expected);
    }
}
