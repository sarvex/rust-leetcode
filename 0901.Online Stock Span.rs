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
struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
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
