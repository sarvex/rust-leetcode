use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Finds the running median using two heaps.
///
/// # Intuition
/// A max-heap holds the smaller half and a min-heap holds the larger half.
/// The median is derived from the tops of both heaps.
///
/// # Approach
/// 1. Push each number through the max-heap, then move its top to the min-heap.
/// 2. If the min-heap grows too large, rebalance by moving its top to the max-heap.
/// 3. The median is the min-heap top (odd count) or the average of both tops (even).
///
/// # Complexity
/// - Time: O(log n) per add, O(1) per find_median
/// - Space: O(n)
struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        if self.min_heap.len() > self.max_heap.len() + 1 {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.min_heap.len() == self.max_heap.len() {
            (self.min_heap.peek().unwrap().0 + self.max_heap.peek().unwrap()) as f64 / 2.0
        } else {
            self.min_heap.peek().unwrap().0 as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streaming_median() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert!((mf.find_median() - 1.5).abs() < f64::EPSILON);
        mf.add_num(3);
        assert!((mf.find_median() - 2.0).abs() < f64::EPSILON);
    }
}
