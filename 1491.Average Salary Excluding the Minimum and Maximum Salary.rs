impl Solution {
    /// Single-pass min/max/sum computation for trimmed average.
    ///
    /// # Intuition
    /// Track the minimum, maximum, and total sum in one pass, then compute
    /// the average excluding the two extremes.
    ///
    /// # Approach
    /// 1. Fold over salaries to find min, max, and sum
    /// 2. Return `(sum - min - max) / (n - 2)`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn average(salary: Vec<i32>) -> f64 {
        let n = salary.len() as i32;
        let (min, max, sum) = salary
            .iter()
            .fold((i32::MAX, i32::MIN, 0), |(mn, mx, s), &v| {
                (mn.min(v), mx.max(v), s + v)
            });
        f64::from(sum - min - max) / f64::from(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_average() {
        assert!((Solution::average(vec![4000, 3000, 1000, 2000]) - 2500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn three_salaries() {
        assert!((Solution::average(vec![1000, 2000, 3000]) - 2000.0).abs() < f64::EPSILON);
    }
}
