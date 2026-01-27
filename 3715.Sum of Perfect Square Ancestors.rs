impl Solution {
    /// Counts ancestor pairs where product forms perfect square
    ///
    /// # Intuition
    /// Product a*b is perfect square iff square-free parts are equal.
    /// Divide each number by its largest square divisor to get square-free part.
    ///
    /// # Approach
    /// 1. Precompute largest perfect square divisor for each value
    /// 2. Transform nums to square-free parts in place
    /// 3. Iterative DFS counting ancestors with matching square-free values
    ///
    /// # Complexity
    /// - Time: O(n + max_value)
    /// - Space: O(n + max_value)
    pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, mut nums: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut adjacency = vec![Vec::new(); n];
        edges.iter().for_each(|edge| {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adjacency[u].push(edge[1]);
            adjacency[v].push(edge[0]);
        });

        let max_value = *nums.iter().max().unwrap() as usize;
        let mut largest_square = vec![1_i32; max_value + 1];
        let mut base = 2;
        loop {
            let square = base * base;
            if square > max_value {
                break;
            }
            let mut multiple = square;
            while multiple <= max_value {
                largest_square[multiple] = largest_square[multiple].max(square as i32);
                multiple += square;
            }
            base += 1;
        }

        nums.iter_mut()
            .for_each(|value| *value /= largest_square[*value as usize]);

        let mut result = 0_i64;
        let mut count = vec![0_i32; max_value + 1];
        let mut stack = Vec::with_capacity(n);
        stack.push((0_i32, 0_i32));

        while let Some((node, parent)) = stack.pop() {
            let index = node as usize;
            if parent != -1 {
                stack.push((node, -1));
                let entry = &mut count[nums[index] as usize];
                result += i64::from(*entry);
                *entry += 1;
                for &child in &adjacency[index] {
                    if child != parent {
                        stack.push((child, node));
                    }
                }
            } else {
                count[nums[index] as usize] -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::sum_of_ancestors(3, vec![vec![0, 1], vec![1, 2]], vec![2, 8, 2]),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::sum_of_ancestors(3, vec![vec![0, 1], vec![0, 2]], vec![1, 2, 4]),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::sum_of_ancestors(
                4,
                vec![vec![0, 1], vec![0, 2], vec![1, 3]],
                vec![1, 2, 9, 4]
            ),
            2
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::sum_of_ancestors(1, vec![], vec![5]), 0);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(
            Solution::sum_of_ancestors(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                vec![1, 1, 1, 1]
            ),
            6
        );
    }
}
