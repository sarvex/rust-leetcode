/// Doubly-linked list with hash map for O(1) LRU cache operations.
///
/// # Intuition
/// An LRU cache requires O(1) lookup by key and O(1) eviction of the least
/// recently used entry. A hash map provides fast lookup while a doubly-linked
/// list maintains access order, enabling O(1) move-to-front and removal.
///
/// # Approach
/// Maintain a hash map from key to `Rc<RefCell<Node>>` and a doubly-linked
/// list with explicit head/tail pointers. On `get`, move the accessed node
/// to the front. On `put`, either update an existing node (and move to front)
/// or insert a new node at the front, evicting the tail if capacity is exceeded.
///
/// # Complexity
/// - Time: O(1) per `get` and `put` operation
/// - Space: O(capacity) — bounded by the cache capacity
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            cache: HashMap::with_capacity(capacity as usize),
            head: None,
            tail: None,
        }
    }

    #[inline]
    fn is_head(&self, node: &Rc<RefCell<Node>>) -> bool {
        self.head
            .as_ref()
            .map_or(false, |head| Rc::ptr_eq(head, node))
    }

    fn get(&mut self, key: i32) -> i32 {
        let node = match self.cache.get(&key) {
            Some(node) => node,
            None => return -1,
        };

        let value = node.borrow().value;
        if self.is_head(node) {
            return value;
        }

        let node = Rc::clone(node);
        self.remove(&node);
        self.push_front(&node);
        value
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow_mut().value = value;
            if self.is_head(node) {
                return;
            }
            let node = Rc::clone(node);
            self.remove(&node);
            self.push_front(&node);
            return;
        }

        let node = Rc::new(RefCell::new(Node::new(key, value)));
        self.push_front(&node);
        self.cache.insert(key, node);

        if self.cache.len() > self.capacity {
            if let Some(evicted) = self.pop_back() {
                self.cache.remove(&evicted.borrow().key);
            }
        }
    }

    #[inline]
    fn push_front(&mut self, node: &Rc<RefCell<Node>>) {
        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::clone(node));
                let mut node_ref = node.borrow_mut();
                node_ref.prev = None;
                node_ref.next = Some(head);
                self.head = Some(Rc::clone(node));
            }
            None => {
                let mut node_ref = node.borrow_mut();
                node_ref.prev = None;
                node_ref.next = None;
                let node_rc = Rc::clone(node);
                self.head = Some(Rc::clone(&node_rc));
                self.tail = Some(node_rc);
            }
        }
    }

    #[inline]
    fn remove(&mut self, node: &Rc<RefCell<Node>>) {
        let (prev, next) = {
            let mut borrowed = node.borrow_mut();
            (borrowed.prev.take(), borrowed.next.take())
        };

        match (prev, next) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next_node)) => {
                next_node.borrow_mut().prev = None;
                self.head = Some(next_node);
            }
            (Some(prev_node), None) => {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            }
            (Some(prev_node), Some(next_node)) => {
                {
                    let mut prev_ref = prev_node.borrow_mut();
                    prev_ref.next = Some(Rc::clone(&next_node));
                }
                next_node.borrow_mut().prev = Some(prev_node);
            }
        }
    }

    #[inline]
    fn pop_back(&mut self) -> Option<Rc<RefCell<Node>>> {
        self.tail.take().map(|tail| {
            self.remove(&tail);
            tail
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_get_and_put() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn update_existing_key() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(1, 20);
        assert_eq!(cache.get(1), 20);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn get_refreshes_recency() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.get(1); // refreshes key 1
        cache.put(3, 3); // evicts key 2 (least recent)
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(1), 1);
    }
}
