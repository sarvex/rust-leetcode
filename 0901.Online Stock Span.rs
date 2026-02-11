struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    /// Monotonic stack-based stock span calculator.
    ///
    /// # Intuition
    /// The span is the number of consecutive days (including today) where the
    /// price was less than or equal to today's price. A decreasing stack of
    /// `(price, span)` pairs efficiently aggregates spans.
    ///
    /// # Approach
    /// On each `next(price)`, pop all entries with price <= current, accumulating
    /// their spans. Push the new entry with the combined span.
    ///
    /// # Complexity
    /// - Time: O(1) amortized per call
    /// - Space: O(n) worst case for the stack
    fn new() -> Self {
        Self {
            stack: vec![(i32::MAX, -1)],
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while self.stack.last().is_some_and(|&(p, _)| p <= price) {
            span += self.stack.pop().unwrap().1;
        }
        self.stack.push((price, span));
        span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut ss = StockSpanner::new();
        assert_eq!(ss.next(100), 1);
        assert_eq!(ss.next(80), 1);
        assert_eq!(ss.next(60), 1);
        assert_eq!(ss.next(70), 2);
        assert_eq!(ss.next(60), 1);
        assert_eq!(ss.next(75), 4);
        assert_eq!(ss.next(85), 6);
    }

    #[test]
    fn all_same_prices() {
        let mut ss = StockSpanner::new();
        for i in 1..=5 {
            assert_eq!(ss.next(50), i);
        }
    }

    #[test]
    fn strictly_increasing() {
        let mut ss = StockSpanner::new();
        assert_eq!(ss.next(1), 1);
        assert_eq!(ss.next(2), 2);
        assert_eq!(ss.next(3), 3);
        assert_eq!(ss.next(4), 4);
    }

    #[test]
    fn strictly_decreasing() {
        let mut ss = StockSpanner::new();
        assert_eq!(ss.next(4), 1);
        assert_eq!(ss.next(3), 1);
        assert_eq!(ss.next(2), 1);
        assert_eq!(ss.next(1), 1);
    }
}
