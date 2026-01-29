/// Circular queue implemented with a fixed-size array.
///
/// # Intuition
/// A circular buffer reuses space by wrapping indices. Using a front pointer
/// and size counter (instead of front/rear) simplifies boundary calculations.
///
/// # Approach
/// 1. Use a Vec as the underlying storage with fixed capacity.
/// 2. Track front index and current size.
/// 3. Compute rear position dynamically as (front + size) % capacity.
/// 4. Enqueue adds at rear, dequeue advances front.
///
/// # Complexity
/// - Time: O(1) for all operations
/// - Space: O(k) where k is the queue capacity
struct MyCircularQueue {
    data: Vec<i32>,
    front: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let cap = k as usize;
        Self {
            data: vec![0; cap],
            front: 0,
            size: 0,
            capacity: cap,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        let rear = (self.front + self.size) % self.capacity;
        self.data[rear] = value;
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.front + self.size - 1) % self.capacity]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut queue = MyCircularQueue::new(3);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(3));
        assert!(!queue.en_queue(4)); // Full
        assert_eq!(queue.rear(), 3);
        assert!(queue.is_full());
        assert!(queue.de_queue());
        assert!(queue.en_queue(4));
        assert_eq!(queue.rear(), 4);
    }

    #[test]
    fn test_empty_queue() {
        let queue = MyCircularQueue::new(3);
        assert!(queue.is_empty());
        assert!(!queue.is_full());
        assert_eq!(queue.front(), -1);
        assert_eq!(queue.rear(), -1);
    }

    #[test]
    fn test_dequeue_empty() {
        let mut queue = MyCircularQueue::new(3);
        assert!(!queue.de_queue());
    }

    #[test]
    fn test_circular_wrap_around() {
        let mut queue = MyCircularQueue::new(3);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(3));
        assert!(queue.de_queue()); // Remove 1
        assert!(queue.de_queue()); // Remove 2
        assert!(queue.en_queue(4)); // Wraps to index 0
        assert!(queue.en_queue(5)); // Wraps to index 1
        assert_eq!(queue.front(), 3);
        assert_eq!(queue.rear(), 5);
    }

    #[test]
    fn test_single_capacity() {
        let mut queue = MyCircularQueue::new(1);
        assert!(queue.en_queue(42));
        assert!(queue.is_full());
        assert_eq!(queue.front(), 42);
        assert_eq!(queue.rear(), 42);
        assert!(!queue.en_queue(100));
        assert!(queue.de_queue());
        assert!(queue.is_empty());
    }

    #[test]
    fn test_alternating_operations() {
        let mut queue = MyCircularQueue::new(2);
        for i in 0..10 {
            assert!(queue.en_queue(i));
            assert_eq!(queue.front(), i);
            assert!(queue.de_queue());
        }
        assert!(queue.is_empty());
    }
}
