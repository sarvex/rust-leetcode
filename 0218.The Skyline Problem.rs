use std::collections::BinaryHeap;

impl Solution {
    /// Computes the skyline silhouette using a sweep line with a max-heap.
    ///
    /// # Intuition
    /// Process vertical sweep lines at building edges. Track active building
    /// heights in a max-heap, removing expired buildings as the line advances.
    ///
    /// # Approach
    /// 1. Collect all x-coordinates (left and right edges) and sort them.
    /// 2. For each sweep line, add buildings that start at or before it.
    /// 3. Remove heap entries that end at or before the current x.
    /// 4. Record a key point when the maximum height changes.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting and heap operations
    /// - Space: O(n) for the heap and result
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lines: Vec<i32> = buildings.iter().flat_map(|b| [b[0], b[1]]).collect();
        lines.sort_unstable();

        let mut heap = BinaryHeap::new();
        let mut result = Vec::new();
        let mut idx = 0;
        let n = buildings.len();

        for &line in &lines {
            while idx < n && buildings[idx][0] <= line && buildings[idx][1] > line {
                heap.push((buildings[idx][2], buildings[idx][1]));
                idx += 1;
            }
            while let Some(&(_, end)) = heap.peek() {
                if end <= line {
                    heap.pop();
                } else {
                    break;
                }
            }
            let height = heap.peek().map_or(0, |&(h, _)| h);
            if result.is_empty() || result.last().unwrap() != &vec![line, height] {
                if result.is_empty() || result.last().unwrap()[1] != height {
                    result.push(vec![line, height]);
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
    fn standard_skyline() {
        let buildings = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ];
        let expected = vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0],
        ];
        assert_eq!(Solution::get_skyline(buildings), expected);
    }

    #[test]
    fn single_building() {
        assert_eq!(
            Solution::get_skyline(vec![vec![0, 2, 3]]),
            vec![vec![0, 3], vec![2, 0]]
        );
    }
}
