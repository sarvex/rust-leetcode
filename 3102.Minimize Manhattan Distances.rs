struct RangeStats {
    max1: i64,
    max2: i64,
    max_count: usize,
    min1: i64,
    min2: i64,
    min_count: usize,
}

impl RangeStats {
    fn from(values: &[i64]) -> Self {
        let mut max1 = i64::MIN;
        let mut max2 = i64::MIN;
        let mut max_count = 0usize;
        let mut min1 = i64::MAX;
        let mut min2 = i64::MAX;
        let mut min_count = 0usize;

        for &value in values {
            if value > max1 {
                max2 = max1;
                max1 = value;
                max_count = 1;
            } else if value == max1 {
                max_count += 1;
            } else if value > max2 {
                max2 = value;
            }

            if value < min1 {
                min2 = min1;
                min1 = value;
                min_count = 1;
            } else if value == min1 {
                min_count += 1;
            } else if value < min2 {
                min2 = value;
            }
        }

        Self {
            max1,
            max2,
            max_count,
            min1,
            min2,
            min_count,
        }
    }
}

impl Solution {
    /// Track extrema in `x + y` and `x - y` after one removal.
    ///
    /// # Intuition
    /// The Manhattan diameter of a point set equals the maximum spread of `x + y` or `x - y`.
    /// Removing one point only changes a spread when that point is the unique extreme.
    ///
    /// # Approach
    /// - Convert each point to `sum = x + y` and `diff = x - y`.
    /// - Precompute max, second max, min, second min, and counts for both transforms.
    /// - For each point, recompute the remaining extrema by substituting second extrema
    ///   when the point uniquely owns an extreme.
    /// - The resulting diameter is `max(sum_range, diff_range)`; take the minimum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut sums = Vec::with_capacity(n);
        let mut diffs = Vec::with_capacity(n);

        for point in points {
            let x = point[0] as i64;
            let y = point[1] as i64;
            sums.push(x + y);
            diffs.push(x - y);
        }

        let sum_stats = RangeStats::from(&sums);
        let diff_stats = RangeStats::from(&diffs);

        let mut best = i64::MAX;

        for idx in 0..n {
            let sum_value = sums[idx];
            let diff_value = diffs[idx];

            let sum_max = if sum_value == sum_stats.max1 && sum_stats.max_count == 1 {
                sum_stats.max2
            } else {
                sum_stats.max1
            };
            let sum_min = if sum_value == sum_stats.min1 && sum_stats.min_count == 1 {
                sum_stats.min2
            } else {
                sum_stats.min1
            };

            let diff_max = if diff_value == diff_stats.max1 && diff_stats.max_count == 1 {
                diff_stats.max2
            } else {
                diff_stats.max1
            };
            let diff_min = if diff_value == diff_stats.min1 && diff_stats.min_count == 1 {
                diff_stats.min2
            } else {
                diff_stats.min1
            };

            let candidate = (sum_max - sum_min).max(diff_max - diff_min);
            if candidate < best {
                best = candidate;
            }
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let points = vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]];
        assert_eq!(Solution::minimum_distance(points), 12);
    }

    #[test]
    fn test_example_2() {
        let points = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::minimum_distance(points), 0);
    }

    #[test]
    fn test_three_points_unique_extreme() {
        let points = vec![vec![1, 1], vec![1, 10], vec![10, 1]];
        assert_eq!(Solution::minimum_distance(points), 9);
    }

    #[test]
    fn test_collinear_points() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::minimum_distance(points), 2);
    }
}
