use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
struct FastHasher(u64);

#[inline]
fn mix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

impl Hasher for FastHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut h = self.0;
        let mut i = 0usize;
        let len = bytes.len();
        while i + 8 <= len {
            let chunk = u64::from_le_bytes([
                bytes[i],
                bytes[i + 1],
                bytes[i + 2],
                bytes[i + 3],
                bytes[i + 4],
                bytes[i + 5],
                bytes[i + 6],
                bytes[i + 7],
            ]);
            h ^= mix64(chunk);
            i += 8;
        }
        if i < len {
            let mut tail = 0u64;
            let mut shift = 0u32;
            while i < len {
                tail |= (bytes[i] as u64) << shift;
                shift += 8;
                i += 1;
            }
            h ^= mix64(tail);
        }
        self.0 = h;
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 ^= mix64(i);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write_u64(i as u64);
        self.write_u64((i >> 64) as u64);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }
}

type BH = BuildHasherDefault<FastHasher>;

impl Solution {
    /// Count pairs of words that are equivalent under cyclic letter shift (Caesar cipher).
    ///
    /// # Intuition
    /// Two strings are similar iff the sequence (char − first_char) mod 26 is identical.
    /// Normalize to that signature; group by signature and count pairs.
    ///
    /// # Approach
    /// Single map keyed by (h1, h2) — 128-bit fingerprint of the normalized string
    /// (polynomial hash, no allocation). Incremental pair count. FastHasher hashes
    /// only two u64s per lookup, so no long-byte hashing or Box for long words.
    ///
    /// # Complexity
    /// - Time: O(n * m) — one pass, zero allocations
    /// - Space: O(n) — map from (u64, u64) to count
    pub fn count_pairs(words: Vec<String>) -> i64 {
        const B1: u64 = 31;
        const B2: u64 = 37;
        let n = words.len();
        let mut freq: HashMap<(u64, u64), i32, BH> =
            HashMap::with_capacity_and_hasher(n, Default::default());
        let mut ans: i64 = 0;

        for w in &words {
            let b = w.as_bytes();
            let first = b[0];
            let (mut h1, mut h2) = (0u64, 0u64);
            for &c in b {
                let d = (c.wrapping_sub(first).wrapping_add(26)) % 26;
                h1 = h1.wrapping_mul(B1).wrapping_add(d as u64);
                h2 = h2.wrapping_mul(B2).wrapping_add(d as u64);
            }
            let e = freq.entry((h1, h2)).or_insert(0);
            ans += *e as i64;
            *e += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_fusion_layout() {
        assert_eq!(Solution::count_pairs(to_vec(&["fusion", "layout"])), 1);
    }

    #[test]
    fn test_ab_aa_za_aa() {
        assert_eq!(Solution::count_pairs(to_vec(&["ab", "aa", "za", "aa"])), 2);
    }

    #[test]
    fn test_single_word() {
        assert_eq!(Solution::count_pairs(to_vec(&["abc"])), 0);
    }

    #[test]
    fn test_all_same_normalized() {
        assert_eq!(Solution::count_pairs(to_vec(&["aa", "bb", "zz"])), 3);
    }
}
