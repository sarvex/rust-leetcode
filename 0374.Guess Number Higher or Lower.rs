// Forward declaration - the actual implementation is provided by the system
extern "C" {
    fn guess(num: i32) -> i32;
}

impl Solution {
    /// Finds the picked number using binary search with the guess API.
    ///
    /// # Intuition
    /// The guess API provides comparison feedback (-1: too high, 1: too low, 0: correct).
    /// This is a classic binary search scenario where we can eliminate half the search
    /// space with each API call.
    ///
    /// # Approach
    /// 1. Initialize search bounds: `left = 1`, `right = n`.
    /// 2. Loop until the number is found:
    ///    - Compute `mid` using overflow-safe arithmetic.
    ///    - Call `guess(mid)` and match on the result:
    ///      - `-1`: guess is too high, search `[left, mid - 1]`
    ///      - `1`: guess is too low, search `[mid + 1, right]`
    ///      - `0`: found the number, return `mid`
    /// 3. The problem guarantees a valid pick exists, so the loop always terminates.
    ///
    /// # Complexity
    /// - Time: O(log n) - binary search halves the search space each iteration
    /// - Space: O(1) - only uses two pointer variables
    #[allow(non_snake_case)]
    pub fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        loop {
            let mid = left + (right - left) / 2;
            match unsafe { guess(mid) } {
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => break mid,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::sync::atomic::{AtomicI32, Ordering as AtomicOrdering};

    // Thread-safe mock for the picked number
    static PICK: AtomicI32 = AtomicI32::new(0);

    fn set_pick(value: i32) {
        PICK.store(value, AtomicOrdering::SeqCst);
    }

    fn guess(num: i32) -> i32 {
        let pick = PICK.load(AtomicOrdering::SeqCst);
        match num.cmp(&pick) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        }
    }

    fn guess_number(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        loop {
            let mid = left + (right - left) / 2;
            match guess(mid) {
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => return mid,
            }
        }
    }

    #[test]
    fn test_guess_middle() {
        set_pick(6);
        assert_eq!(guess_number(10), 6);
    }

    #[test]
    fn test_guess_first() {
        set_pick(1);
        assert_eq!(guess_number(1), 1);
        assert_eq!(guess_number(10), 1);
    }

    #[test]
    fn test_guess_last() {
        set_pick(10);
        assert_eq!(guess_number(10), 10);
    }

    #[test]
    fn test_guess_two_elements() {
        set_pick(1);
        assert_eq!(guess_number(2), 1);

        set_pick(2);
        assert_eq!(guess_number(2), 2);
    }

    #[test]
    fn test_large_range() {
        set_pick(1_234_567);
        assert_eq!(guess_number(2_000_000_000), 1_234_567);
    }

    #[test]
    fn test_pick_near_max() {
        set_pick(i32::MAX - 1);
        assert_eq!(guess_number(i32::MAX), i32::MAX - 1);
    }
}
