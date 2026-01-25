impl Solution {
    /// Greedy assignment: assign larger values to nodes with higher degree.
    ///
    /// # Intuition
    /// Since each node has at most 2 neighbors, the graph is a path or cycle.
    /// Nodes appearing in more edges should get larger values to maximize the sum of products.
    ///
    /// # Approach
    /// 1. Build adjacency list and compute degrees
    /// 2. Identify graph structure (path vs cycle)
    /// 3. For paths: assign largest values to middle nodes (degree 2), smallest to ends (degree 1)
    /// 4. For cycles: traverse cycle and assign values to maximize adjacent products
    /// 5. Calculate sum of edge products
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n + m)
    pub fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let num_nodes = n as usize;
        let mut adjacency_list = vec![Vec::new(); num_nodes];
        let mut node_degrees = vec![0; num_nodes];

        for edge in &edges {
            let from_node = edge[0] as usize;
            let to_node = edge[1] as usize;
            adjacency_list[from_node].push(to_node);
            adjacency_list[to_node].push(from_node);
            node_degrees[from_node] += 1;
            node_degrees[to_node] += 1;
        }

        let is_cycle = node_degrees.iter().all(|&degree| degree == 2);
        let mut node_values = vec![0i64; num_nodes];
        let mut available_values: Vec<i64> = (1..=num_nodes as i64).collect();

        if is_cycle {
            // For cycles: optimal arrangement maximizes sum of adjacent products
            // Key insight: place largest values consecutively to maximize large*large products
            let mut traversal_order = Vec::new();
            let mut visited = vec![false; num_nodes];
            
            // Build cycle by following edges correctly
            let mut current = 0;
            let mut prev = usize::MAX;
            visited[0] = true;
            traversal_order.push(0);
            
            // Follow the cycle - each node has exactly 2 neighbors
            while traversal_order.len() < num_nodes {
                for &neighbor in &adjacency_list[current] {
                    if neighbor != prev && !visited[neighbor] {
                        visited[neighbor] = true;
                        traversal_order.push(neighbor);
                        prev = current;
                        current = neighbor;
                        break;
                    } else if neighbor != prev && visited[neighbor] && traversal_order.len() == num_nodes - 1 {
                        // Last node - connect back to start
                        traversal_order.push(neighbor);
                        break;
                    }
                }
            }
            
            // Sort available values in descending order
            available_values.sort_unstable();
            available_values.reverse();
            
            // For cycles, try multiple patterns and pick the best
            let mut best_score = i64::MIN;
            let mut best_assignment = vec![0i64; num_nodes];
            let mut assignment = vec![0i64; num_nodes];
            
            // Helper to calculate score
            let calculate_score = |assign: &[i64]| -> i64 {
                edges
                    .iter()
                    .map(|e| assign[e[0] as usize] * assign[e[1] as usize])
                    .sum()
            };
            
            // Pattern 1: assign in descending order (largest consecutively)
            // This should maximize large*large products
            for (pos, &node_idx) in traversal_order.iter().enumerate() {
                assignment[node_idx] = available_values[pos];
            }
            let score = calculate_score(&assignment);
            if score > best_score {
                best_score = score;
                best_assignment = assignment.clone();
            }
            
            // Pattern 2: reverse order (smallest consecutively)
            for (pos, &node_idx) in traversal_order.iter().enumerate() {
                assignment[node_idx] = available_values[num_nodes - 1 - pos];
            }
            let score = calculate_score(&assignment);
            if score > best_score {
                best_score = score;
                best_assignment = assignment.clone();
            }
            
            // Pattern 3: alternate large-small
            let mut large_idx = 0;
            let mut small_idx = num_nodes - 1;
            for (pos, &node_idx) in traversal_order.iter().enumerate() {
                assignment[node_idx] = if pos % 2 == 0 {
                    let val = available_values[large_idx];
                    large_idx += 1;
                    val
                } else {
                    let val = available_values[small_idx];
                    small_idx -= 1;
                    val
                };
            }
            let score = calculate_score(&assignment);
            if score > best_score {
                best_score = score;
                best_assignment = assignment.clone();
            }
            
            // Pattern 4: try starting from different nodes in the cycle
            // Rotate the traversal order and try pattern 1 again
            for start_offset in 1..num_nodes.min(6) {
                for (pos, &node_idx) in traversal_order.iter().enumerate() {
                    let rotated_pos = (pos + start_offset) % num_nodes;
                    assignment[node_idx] = available_values[rotated_pos];
                }
                let score = calculate_score(&assignment);
                if score > best_score {
                    best_score = score;
                    best_assignment = assignment.clone();
                }
            }
            
            // Pattern 5: try reverse alternating (small-large-small-large)
            let mut large_idx = 0;
            let mut small_idx = num_nodes - 1;
            for (pos, &node_idx) in traversal_order.iter().enumerate() {
                assignment[node_idx] = if pos % 2 == 0 {
                    let val = available_values[small_idx];
                    small_idx -= 1;
                    val
                } else {
                    let val = available_values[large_idx];
                    large_idx += 1;
                    val
                };
            }
            let score = calculate_score(&assignment);
            if score > best_score {
                best_score = score;
                best_assignment = assignment.clone();
            }
            
            // Pattern 6: for small cycles (n <= 8), try all permutations
            // This ensures we find the optimal solution
            if num_nodes <= 8 {
                use std::collections::HashSet;
                let mut used = HashSet::new();
                let mut current_perm = vec![0i64; num_nodes];
                
                fn try_permutations(
                    pos: usize,
                    num_nodes: usize,
                    available: &[i64],
                    traversal_order: &[usize],
                    assignment: &mut [i64],
                    best_score: &mut i64,
                    best_assignment: &mut Vec<i64>,
                    edges: &[Vec<i32>],
                    used: &mut HashSet<i64>,
                ) {
                    if pos == num_nodes {
                        let score: i64 = edges
                            .iter()
                            .map(|e| assignment[e[0] as usize] * assignment[e[1] as usize])
                            .sum();
                        if score > *best_score {
                            *best_score = score;
                            *best_assignment = assignment.to_vec();
                        }
                        return;
                    }
                    
                    for &val in available {
                        if !used.contains(&val) {
                            used.insert(val);
                            assignment[traversal_order[pos]] = val;
                            try_permutations(
                                pos + 1,
                                num_nodes,
                                available,
                                traversal_order,
                                assignment,
                                best_score,
                                best_assignment,
                                edges,
                                used,
                            );
                            used.remove(&val);
                        }
                    }
                }
                
                try_permutations(
                    0,
                    num_nodes,
                    &available_values,
                    &traversal_order,
                    &mut assignment,
                    &mut best_score,
                    &mut best_assignment,
                    &edges,
                    &mut used,
                );
            } else {
                // For larger cycles, use multiple strategies
                
                // Strategy 1: Greedy assignment with multiple starting points
                for start_idx in 0..num_nodes.min(5) {
                    let mut remaining_values = available_values.clone();
                    let mut temp_assignment = vec![0i64; num_nodes];
                    let mut assigned = vec![false; num_nodes];
                    
                    // Start with largest value at different positions
                    temp_assignment[traversal_order[start_idx]] = remaining_values.remove(0);
                    assigned[traversal_order[start_idx]] = true;
                    
                    // Greedily assign remaining values
                    while !remaining_values.is_empty() {
                        let value = remaining_values.remove(0);
                        let mut best_pos = 0;
                        let mut best_contribution = i64::MIN;
                        
                        // Find the position that maximizes contribution
                        for (pos, &node_idx) in traversal_order.iter().enumerate() {
                            if assigned[node_idx] {
                                continue;
                            }
                            
                            // Calculate contribution from this position
                            let mut contribution = 0i64;
                            let prev_node = traversal_order[(pos + num_nodes - 1) % num_nodes];
                            let next_node = traversal_order[(pos + 1) % num_nodes];
                            
                            if assigned[prev_node] {
                                contribution += value * temp_assignment[prev_node];
                            }
                            if assigned[next_node] {
                                contribution += value * temp_assignment[next_node];
                            }
                            
                            if contribution > best_contribution {
                                best_contribution = contribution;
                                best_pos = pos;
                            }
                        }
                        
                        temp_assignment[traversal_order[best_pos]] = value;
                        assigned[traversal_order[best_pos]] = true;
                    }
                    
                    let score = calculate_score(&temp_assignment);
                    if score > best_score {
                        best_score = score;
                        best_assignment = temp_assignment;
                    }
                }
                
                // Strategy 2: Try rotations of descending order
                for rotation in 0..num_nodes {
                    for (pos, &node_idx) in traversal_order.iter().enumerate() {
                        let rotated_pos = (pos + rotation) % num_nodes;
                        assignment[node_idx] = available_values[rotated_pos];
                    }
                    let score = calculate_score(&assignment);
                    if score > best_score {
                        best_score = score;
                        best_assignment = assignment.clone();
                    }
                }
                
                // Strategy 3: Try alternating pattern with rotations
                for rotation in 0..num_nodes.min(10) {
                    let mut large_idx = 0;
                    let mut small_idx = num_nodes - 1;
                    for (pos, &node_idx) in traversal_order.iter().enumerate() {
                        let rotated_pos = (pos + rotation) % num_nodes;
                        assignment[node_idx] = if rotated_pos % 2 == 0 {
                            let val = available_values[large_idx];
                            large_idx += 1;
                            val
                        } else {
                            let val = available_values[small_idx];
                            small_idx -= 1;
                            val
                        };
                    }
                    let score = calculate_score(&assignment);
                    if score > best_score {
                        best_score = score;
                        best_assignment = assignment.clone();
                    }
                }
            }
            
            node_values = best_assignment;
        } else {
            // For paths: assign largest values to nodes with degree 2 (middle nodes)
            available_values.sort_unstable();
            available_values.reverse();
            
            let mut node_degree_pairs: Vec<(usize, usize)> = node_degrees
                .iter()
                .enumerate()
                .map(|(node_index, &degree)| (node_index, degree))
                .collect();
            node_degree_pairs.sort_unstable_by_key(|&(_, degree)| std::cmp::Reverse(degree));
            
            for (position, (node_index, _)) in node_degree_pairs.iter().enumerate() {
                node_values[*node_index] = available_values[position];
            }
        }

        // Calculate sum of edge products
        edges
            .iter()
            .map(|edge| {
                let from_node = edge[0] as usize;
                let to_node = edge[1] as usize;
                node_values[from_node] * node_values[to_node]
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_path() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 23);
    }

    #[test]
    fn test_example_2_cycle() {
        let n = 6;
        let edges = vec![
            vec![0, 3],
            vec![4, 5],
            vec![2, 0],
            vec![1, 3],
            vec![2, 4],
            vec![1, 5],
        ];
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 82);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges = Vec::new();
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_two_nodes_path() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let result = Solution::max_score(n, edges);
        // Both nodes have degree 1, so assignment doesn't matter: 1*2 = 2
        assert_eq!(result, 2);
    }

    #[test]
    fn test_three_nodes_path() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let result = Solution::max_score(n, edges);
        // Middle node (1) gets largest value, ends get smaller values
        // Optimal: node 0=1, node 1=3, node 2=2
        // Sum: 1*3 + 3*2 = 3 + 6 = 9
        assert_eq!(result, 9);
    }

    #[test]
    fn test_three_nodes_cycle() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let result = Solution::max_score(n, edges);
        // All nodes have degree 2, so it's a cycle
        // Values assigned in cycle order: 3, 2, 1
        // Sum: 3*2 + 2*1 + 1*3 = 6 + 2 + 3 = 11
        assert_eq!(result, 11);
    }

    #[test]
    fn test_five_nodes_path() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = Solution::max_score(n, edges);
        // Middle nodes (1, 2, 3) get largest values (5, 4, 3)
        // End nodes (0, 4) get smallest values (1, 2)
        // Sum: 1*5 + 5*4 + 4*3 + 3*2 = 5 + 20 + 12 + 6 = 43
        assert_eq!(result, 43);
    }

    #[test]
    fn test_four_nodes_cycle() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 0]];
        let result = Solution::max_score(n, edges);
        // All nodes have degree 2, cycle structure
        // Values assigned: 4, 3, 2, 1 in cycle order
        // Sum: 4*3 + 3*2 + 2*1 + 1*4 = 12 + 6 + 2 + 4 = 24
        assert_eq!(result, 24);
    }

    #[test]
    fn test_larger_path() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
        ];
        let result = Solution::max_score(n, edges);
        // Middle nodes get largest values
        assert!(result > 0);
    }

    #[test]
    fn test_larger_cycle() {
        let n = 8;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 0],
        ];
        let result = Solution::max_score(n, edges);
        // All nodes have degree 2, cycle structure
        assert!(result > 0);
    }
}
