/// Ordered stream that emits consecutive values from a pointer.
///
/// # Intuition
/// Store values at their designated positions. On each insert, if the
/// pointer's position is filled, emit all consecutive filled values.
///
/// # Complexity
/// - insert: O(k) where k is the number of consecutive values emitted
/// - Space: O(n)
struct OrderedStream {
    ptr: usize,
    data: Vec<Option<String>>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            ptr: 1,
            data: vec![None; (n + 1) as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.data[id_key as usize] = Some(value);
        let mut result = Vec::new();
        while self.ptr < self.data.len() && self.data[self.ptr].is_some() {
            result.push(self.data[self.ptr].take().unwrap());
            self.ptr += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_stream_operations() {
        let mut os = OrderedStream::new(5);
        assert_eq!(os.insert(3, "c".to_string()), Vec::<String>::new());
        assert_eq!(os.insert(1, "a".to_string()), vec!["a"]);
        assert_eq!(os.insert(2, "b".to_string()), vec!["b", "c"]);
        assert_eq!(os.insert(5, "e".to_string()), Vec::<String>::new());
        assert_eq!(os.insert(4, "d".to_string()), vec!["d", "e"]);
    }
}
