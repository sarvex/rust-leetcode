use std::collections::HashMap;

struct RangeFreqQuery {
    indices: HashMap<i32, Vec<usize>>,
}

impl RangeFreqQuery {
    /// Range frequency query structure using sorted index lists per value.
    ///
    /// # Intuition
    /// Precompute sorted index lists per value so that range frequency queries
    /// reduce to counting indices within `[left, right]` via binary search.
    ///
    /// # Approach
    /// During construction, build a HashMap mapping each value to a sorted
    /// vector of its indices. For each query, binary search for the lower and
    /// upper bounds within the index list to count occurrences in O(log n).
    ///
    /// # Complexity
    /// - Construction: O(n)
    /// - Query: O(log n) per call
    /// - Space: O(n)
    fn new(arr: Vec<i32>) -> Self {
        let indices = arr
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, &val)| {
                acc.entry(val).or_insert_with(Vec::new).push(i);
                acc
            });
        Self { indices }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        self.indices.get(&value).map_or(0, |idx| {
            let lo = idx.partition_point(|&x| x < left as usize);
            let hi = idx.partition_point(|&x| x <= right as usize);
            (hi - lo) as i32
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_frequency() {
        let rq = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
        assert_eq!(rq.query(1, 2, 4), 1);
        assert_eq!(rq.query(0, 11, 33), 2);
    }
}
