/// # Design a Text Editor
///
/// Gap buffer implementation for efficient cursor-based text editing operations.
///
/// # Intuition
/// A text editor with cursor support requires efficient insertion, deletion,
/// and cursor movement. The gap buffer pattern using two stacks (left and right
/// of cursor) provides O(1) amortized operations for all required functionality.
///
/// # Approach
/// - Use `Vec<u8>` for single-byte ASCII operations (per LeetCode constraints)
/// - Leverage `drain().rev()` for zero-allocation transfers (Drain is DoubleEndedIterator)
/// - All operations use safe Rust with no unsafe blocks
///
/// # Complexity
/// - Time: O(k) for cursor movement, O(n) for text insertion
/// - Space: O(n) total, O(1) auxiliary for cursor operations

const PREVIEW_LEN: usize = 10;
const DEFAULT_CAPACITY: usize = 128;

/// High-performance text editor using gap buffer architecture.
///
/// The editor maintains text in two buffers split at the cursor position,
/// enabling O(1) amortized time for insertions and deletions at cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextEditor {
    /// Bytes to the left of cursor (in natural reading order)
    left: Vec<u8>,
    /// Bytes to the right of cursor (in reversed order for stack semantics)
    right: Vec<u8>,
}

impl Default for TextEditor {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditor {
    /// Creates a new empty text editor with pre-allocated capacity.
    ///
    /// # Examples
    /// ```
    /// let editor = TextEditor::new();
    /// assert!(editor.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            left: Vec::with_capacity(DEFAULT_CAPACITY),
            right: Vec::with_capacity(DEFAULT_CAPACITY / 4),
        }
    }

    /// Creates a text editor with specified initial capacity.
    ///
    /// Use when approximate text size is known to minimize reallocations.
    ///
    /// # Arguments
    /// * `capacity` - Initial capacity for the left buffer
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            left: Vec::with_capacity(capacity),
            right: Vec::with_capacity(capacity / 4),
        }
    }

    /// Appends text at the current cursor position.
    ///
    /// Characters are inserted to the left of the cursor, which remains
    /// at the end of the inserted text.
    ///
    /// # Arguments
    /// * `text` - The text to insert at cursor position
    ///
    /// # Complexity
    /// - Time: O(n) where n is text length
    /// - Space: O(1) amortized
    #[inline]
    pub fn add_text(&mut self, text: String) {
        self.left.extend_from_slice(text.as_bytes());
    }

    /// Deletes up to `k` characters to the left of the cursor.
    ///
    /// Returns the actual number of characters deleted, which may be less
    /// than `k` if fewer characters exist to the left of cursor.
    ///
    /// # Arguments
    /// * `k` - Maximum number of characters to delete
    ///
    /// # Returns
    /// The actual count of deleted characters.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    #[inline]
    pub fn delete_text(&mut self, k: i32) -> i32 {
        let delete_count = (k as usize).min(self.left.len());
        self.left.truncate(self.left.len() - delete_count);
        delete_count as i32
    }

    /// Moves the cursor `k` positions to the left.
    ///
    /// Returns the last 10 characters to the left of cursor after movement.
    /// If fewer than `k` characters exist, cursor moves to the beginning.
    ///
    /// # Arguments
    /// * `k` - Number of positions to move left
    ///
    /// # Returns
    /// Up to 10 characters immediately left of cursor after movement.
    ///
    /// # Complexity
    /// - Time: O(k)
    /// - Space: O(1) auxiliary (drain+rev is zero-allocation)
    #[inline]
    pub fn cursor_left(&mut self, k: i32) -> String {
        let move_count = (k as usize).min(self.left.len());

        if move_count > 0 {
            let split_point = self.left.len() - move_count;
            self.right.reserve(move_count);
            // drain().rev() is O(1) space - Drain implements DoubleEndedIterator
            self.right.extend(self.left.drain(split_point..).rev());
        }

        self.extract_preview()
    }

    /// Moves the cursor `k` positions to the right.
    ///
    /// Returns the last 10 characters to the left of cursor after movement.
    /// If fewer than `k` characters exist to the right, cursor moves to the end.
    ///
    /// # Arguments
    /// * `k` - Number of positions to move right
    ///
    /// # Returns
    /// Up to 10 characters immediately left of cursor after movement.
    ///
    /// # Complexity
    /// - Time: O(k)
    /// - Space: O(1) auxiliary (drain+rev is zero-allocation)
    #[inline]
    pub fn cursor_right(&mut self, k: i32) -> String {
        let move_count = (k as usize).min(self.right.len());

        if move_count > 0 {
            let split_point = self.right.len() - move_count;
            self.left.reserve(move_count);
            // drain().rev() restores natural order when moving from right to left
            self.left.extend(self.right.drain(split_point..).rev());
        }

        self.extract_preview()
    }

    /// Extracts up to 10 characters immediately left of cursor for preview.
    ///
    /// This is the standard return value for cursor movement operations.
    #[inline]
    fn extract_preview(&self) -> String {
        let len = self.left.len();
        let start = len.saturating_sub(PREVIEW_LEN);
        // Input is guaranteed to be lowercase English letters (valid ASCII/UTF-8)
        String::from_utf8_lossy(&self.left[start..]).into_owned()
    }

    /// Returns the total number of characters in the editor.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.left.len() + self.right.len()
    }

    /// Returns whether the editor contains no text.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }

    /// Returns the current cursor position (0-indexed from start).
    #[inline]
    #[must_use]
    pub fn cursor_position(&self) -> usize {
        self.left.len()
    }

    /// Returns the complete text content of the editor.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    #[must_use]
    pub fn get_text(&self) -> String {
        let mut result = String::with_capacity(self.len());
        result.push_str(&String::from_utf8_lossy(&self.left));
        // Right buffer is reversed, so iterate backwards
        for &byte in self.right.iter().rev() {
            result.push(byte as char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_editor_is_empty() {
        let editor = TextEditor::new();

        assert!(editor.is_empty());
        assert_eq!(editor.len(), 0);
        assert_eq!(editor.cursor_position(), 0);
    }

    #[test]
    fn test_default_trait() {
        let editor = TextEditor::default();

        assert!(editor.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let editor = TextEditor::with_capacity(1000);

        assert!(editor.is_empty());
    }

    #[test]
    fn test_add_text_single() {
        let mut editor = TextEditor::new();

        editor.add_text("hello".into());

        assert_eq!(editor.len(), 5);
        assert_eq!(editor.cursor_position(), 5);
        assert!(!editor.is_empty());
    }

    #[test]
    fn test_add_text_multiple() {
        let mut editor = TextEditor::new();

        editor.add_text("hello".into());
        editor.add_text("world".into());

        assert_eq!(editor.len(), 10);
        assert_eq!(editor.cursor_position(), 10);
        assert_eq!(editor.get_text(), "helloworld");
    }

    #[test]
    fn test_delete_text_partial() {
        let mut editor = TextEditor::new();
        editor.add_text("hello".into());

        let deleted = editor.delete_text(2);

        assert_eq!(deleted, 2);
        assert_eq!(editor.len(), 3);
        assert_eq!(editor.get_text(), "hel");
    }

    #[test]
    fn test_delete_text_exceeds_available() {
        let mut editor = TextEditor::new();
        editor.add_text("hi".into());

        let deleted = editor.delete_text(10);

        assert_eq!(deleted, 2);
        assert!(editor.is_empty());
    }

    #[test]
    fn test_delete_text_empty_editor() {
        let mut editor = TextEditor::new();

        let deleted = editor.delete_text(5);

        assert_eq!(deleted, 0);
    }

    #[test]
    fn test_cursor_left_returns_preview() {
        let mut editor = TextEditor::new();
        editor.add_text("abcdefghijklmno".into());

        let preview = editor.cursor_left(5);

        assert_eq!(preview, "abcdefghij");
        assert_eq!(editor.cursor_position(), 10);
    }

    #[test]
    fn test_cursor_left_short_text() {
        let mut editor = TextEditor::new();
        editor.add_text("abc".into());

        let preview = editor.cursor_left(1);

        assert_eq!(preview, "ab");
        assert_eq!(editor.cursor_position(), 2);
    }

    #[test]
    fn test_cursor_left_exceeds_available() {
        let mut editor = TextEditor::new();
        editor.add_text("abc".into());

        let preview = editor.cursor_left(10);

        assert_eq!(preview, "");
        assert_eq!(editor.cursor_position(), 0);
    }

    #[test]
    fn test_cursor_left_empty_editor() {
        let mut editor = TextEditor::new();

        let preview = editor.cursor_left(5);

        assert_eq!(preview, "");
        assert_eq!(editor.cursor_position(), 0);
    }

    #[test]
    fn test_cursor_right_returns_preview() {
        let mut editor = TextEditor::new();
        editor.add_text("abcdefghijklmno".into());
        editor.cursor_left(10);

        let preview = editor.cursor_right(5);

        assert_eq!(preview, "abcdefghij");
        assert_eq!(editor.cursor_position(), 10);
    }

    #[test]
    fn test_cursor_right_exceeds_available() {
        let mut editor = TextEditor::new();
        editor.add_text("abc".into());
        editor.cursor_left(3);

        let preview = editor.cursor_right(10);

        assert_eq!(preview, "abc");
        assert_eq!(editor.cursor_position(), 3);
    }

    #[test]
    fn test_cursor_right_empty_editor() {
        let mut editor = TextEditor::new();

        let preview = editor.cursor_right(5);

        assert_eq!(preview, "");
        assert_eq!(editor.cursor_position(), 0);
    }

    #[test]
    fn test_leetcode_example() {
        let mut editor = TextEditor::new();

        editor.add_text("leetcode".into());
        assert_eq!(editor.cursor_position(), 8);

        let deleted = editor.delete_text(4);
        assert_eq!(deleted, 4);

        editor.add_text("practice".into());

        let preview = editor.cursor_right(3);
        assert_eq!(preview, "etpractice");

        let preview = editor.cursor_left(8);
        assert_eq!(preview, "leet");

        let deleted = editor.delete_text(10);
        assert_eq!(deleted, 4);

        let preview = editor.cursor_left(2);
        assert_eq!(preview, "");

        let preview = editor.cursor_right(6);
        assert_eq!(preview, "practi");
    }

    #[test]
    fn test_interleaved_operations() {
        let mut editor = TextEditor::new();

        editor.add_text("abcde".into());
        editor.cursor_left(2);
        editor.add_text("XY".into());

        let preview = editor.cursor_right(2);

        assert_eq!(preview, "abcXYde");
        assert_eq!(editor.len(), 7);
        assert_eq!(editor.get_text(), "abcXYde");
    }

    #[test]
    fn test_preview_exactly_10_chars() {
        let mut editor = TextEditor::new();
        editor.add_text("1234567890".into());

        let preview = editor.cursor_left(0);

        assert_eq!(preview, "1234567890");
        assert_eq!(preview.len(), 10);
    }

    #[test]
    fn test_preview_more_than_10_chars() {
        let mut editor = TextEditor::new();
        editor.add_text("123456789012345".into());

        let preview = editor.cursor_left(0);

        assert_eq!(preview, "6789012345");
        assert_eq!(preview.len(), 10);
    }

    #[test]
    fn test_preview_less_than_10_chars() {
        let mut editor = TextEditor::new();
        editor.add_text("abc".into());

        let preview = editor.cursor_left(0);

        assert_eq!(preview, "abc");
    }

    #[test]
    fn test_clone_trait() {
        let mut editor = TextEditor::new();
        editor.add_text("test".into());
        editor.cursor_left(2);

        let cloned = editor.clone();

        assert_eq!(cloned.len(), editor.len());
        assert_eq!(cloned.cursor_position(), editor.cursor_position());
        assert_eq!(cloned.get_text(), editor.get_text());
    }

    #[test]
    fn test_single_char_movements() {
        let mut editor = TextEditor::new();
        editor.add_text("abcde".into());

        for _ in 0..5 {
            editor.cursor_left(1);
        }
        assert_eq!(editor.cursor_position(), 0);

        for _ in 0..5 {
            editor.cursor_right(1);
        }
        assert_eq!(editor.cursor_position(), 5);
        assert_eq!(editor.get_text(), "abcde");
    }

    #[test]
    fn test_delete_at_middle() {
        let mut editor = TextEditor::new();
        editor.add_text("abcdef".into());
        editor.cursor_left(3);

        let deleted = editor.delete_text(2);

        assert_eq!(deleted, 2);
        assert_eq!(editor.get_text(), "adef");
        assert_eq!(editor.cursor_position(), 1);
    }

    #[test]
    fn test_eq_trait() {
        let mut editor1 = TextEditor::new();
        let mut editor2 = TextEditor::new();

        editor1.add_text("test".into());
        editor2.add_text("test".into());

        assert_eq!(editor1, editor2);
    }
}
