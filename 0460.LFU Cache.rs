use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    freq: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            freq: 1,
            prev: None,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, node: &Rc<RefCell<Node>>) {
        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::clone(node));
                node.borrow_mut().prev = None;
                node.borrow_mut().next = Some(head);
                self.head = Some(Rc::clone(node));
            }
            None => {
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.head = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            }
        }
    }

    fn remove(&mut self, node: &Rc<RefCell<Node>>) {
        match (node.borrow().prev.as_ref(), node.borrow().next.as_ref()) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                self.head = Some(Rc::clone(next));
                next.borrow_mut().prev = None;
            }
            (Some(prev), None) => {
                self.tail = Some(Rc::clone(prev));
                prev.borrow_mut().next = None;
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev = Some(Rc::clone(prev));
                prev.borrow_mut().next = Some(Rc::clone(next));
            }
        }
    }

    fn pop_back(&mut self) -> Option<Rc<RefCell<Node>>> {
        self.tail.take().map(|tail| {
            self.remove(&tail);
            tail
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

struct LFUCache {
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    freq_map: HashMap<i32, LinkedList>,
    min_freq: i32,
    capacity: usize,
}

impl LFUCache {
    /// LFU (Least Frequently Used) cache with O(1) get and put operations.
    ///
    /// # Intuition
    /// Combine a hash map for O(1) key lookup with frequency-bucketed doubly-linked
    /// lists for O(1) eviction. Track the minimum frequency to find the LFU victim.
    ///
    /// # Approach
    /// 1. Store nodes in a HashMap for O(1) access by key.
    /// 2. Group nodes by frequency in separate doubly-linked lists.
    /// 3. On access, move the node to the next frequency bucket.
    /// 4. On eviction, remove from the tail of the min-frequency list.
    ///
    /// # Complexity
    /// - Time: O(1) for both get and put operations
    /// - Space: O(capacity) for storing cache entries
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            cache: HashMap::with_capacity(capacity),
            freq_map: HashMap::with_capacity(capacity),
            min_freq: 0,
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        match self.cache.get(&key) {
            Some(node) => {
                let node = Rc::clone(node);
                self.incr_freq(&node);
                node.borrow().value
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        match self.cache.get(&key) {
            Some(node) => {
                let node = Rc::clone(node);
                node.borrow_mut().value = value;
                self.incr_freq(&node);
            }
            None => {
                if self.cache.len() == self.capacity {
                    let list = self.freq_map.get_mut(&self.min_freq).unwrap();
                    self.cache.remove(&list.pop_back().unwrap().borrow().key);
                }
                let node = Rc::new(RefCell::new(Node::new(key, value)));
                self.add_node(&node);
                self.cache.insert(key, node);
                self.min_freq = 1;
            }
        }
    }

    fn incr_freq(&mut self, node: &Rc<RefCell<Node>>) {
        let freq = node.borrow().freq;
        let list = self.freq_map.get_mut(&freq).unwrap();
        list.remove(node);
        if list.is_empty() {
            self.freq_map.remove(&freq);
            if freq == self.min_freq {
                self.min_freq += 1;
            }
        }
        node.borrow_mut().freq += 1;
        self.add_node(node);
    }

    fn add_node(&mut self, node: &Rc<RefCell<Node>>) {
        let freq = node.borrow().freq;
        self.freq_map
            .entry(freq)
            .or_insert_with(LinkedList::new)
            .push_front(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfu_cache_basic() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
        cache.put(4, 4); // evicts key 1 (or 3, depends on LRU within same freq)
                         // After put(3,3): freq(1)=2, freq(3)=1
                         // After get(3): freq(1)=2, freq(3)=2
                         // So put(4,4) should evict key 1 (older with same freq)
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_lfu_cache_zero_capacity() {
        let mut cache = LFUCache::new(0);
        cache.put(0, 0);
        assert_eq!(cache.get(0), -1);
    }

    #[test]
    fn test_lfu_cache_update_value() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 1);
        cache.put(1, 10);
        assert_eq!(cache.get(1), 10);
    }

    #[test]
    fn test_lfu_cache_frequency_ordering() {
        let mut cache = LFUCache::new(3);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        assert_eq!(cache.get(1), 1); // freq(1) = 2
        assert_eq!(cache.get(1), 1); // freq(1) = 3
        assert_eq!(cache.get(2), 2); // freq(2) = 2
        cache.put(4, 4); // evicts key 3 (freq = 1)
        assert_eq!(cache.get(3), -1);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_lfu_cache_single_capacity() {
        let mut cache = LFUCache::new(1);
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }
}
