use std::collections::BTreeSet;

impl Solution {
    /// Sorted unique words from brace unions and concatenation.
    ///
    /// # Intuition
    /// A comma inside braces is a union: each alternative is its own set of words.
    /// Writing two expressions next to each other is concatenation: every word from
    /// the left paired with every word from the right. Nested braces are the same
    /// two rules on a smaller piece of the expression.
    ///
    /// # Approach
    /// Recursive descent over the expression bytes:
    /// 1. An expression is one or more factors placed side by side, and it stops at
    ///    `,`, `}`, or the end of the string.
    /// 2. A factor is either a run of letters or a `{...}` group.
    /// 3. A group parses comma-separated expressions and unions their word sets.
    /// 4. Side-by-side factors combine by Cartesian product, concatenating each pair.
    ///
    /// Every intermediate result is a `BTreeSet`, so repeated words are dropped as
    /// soon as they appear and the final iteration order is lexicographical.
    ///
    /// # Complexity
    /// - Time: O(P · L log K), where `K` is the number of distinct words, `L` is
    ///   their length, and `P` is the number of pairs formed by concatenation
    /// - Space: O(K · L)
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let (words, _) = parse_expr(expression.as_bytes(), 0);
        words.into_iter().collect()
    }
}

/// Factors concatenated until a comma, a closing brace, or the end of the input.
fn parse_expr(bytes: &[u8], mut index: usize) -> (BTreeSet<String>, usize) {
    let (mut words, next) = parse_factor(bytes, index);
    index = next;
    while index < bytes.len() && bytes[index] != b',' && bytes[index] != b'}' {
        let (factor, next) = parse_factor(bytes, index);
        words = concatenate(words, &factor);
        index = next;
    }
    (words, index)
}

/// A run of letters, or a brace group of comma-separated expressions.
fn parse_factor(bytes: &[u8], index: usize) -> (BTreeSet<String>, usize) {
    if bytes[index] == b'{' {
        return parse_group(bytes, index + 1);
    }

    let end = bytes[index..]
        .iter()
        .position(|byte| !byte.is_ascii_lowercase())
        .map_or(bytes.len(), |offset| index + offset);
    let word: String = bytes[index..end]
        .iter()
        .map(|byte| char::from(*byte))
        .collect();
    (BTreeSet::from([word]), end)
}

/// Union of the expressions inside `{...}`. `index` points just past `{`.
fn parse_group(bytes: &[u8], mut index: usize) -> (BTreeSet<String>, usize) {
    let mut union = BTreeSet::new();
    loop {
        let (part, next) = parse_expr(bytes, index);
        union.extend(part);
        match bytes[next] {
            b'}' => return (union, next + 1),
            _ => index = next + 1,
        }
    }
}

/// Every word of `left` concatenated with every word of `right`.
fn concatenate(left: BTreeSet<String>, right: &BTreeSet<String>) -> BTreeSet<String> {
    left.into_iter()
        .flat_map(|prefix| {
            right.iter().map(move |suffix| {
                let mut word = String::with_capacity(prefix.len() + suffix.len());
                word.push_str(&prefix);
                word.push_str(suffix);
                word
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand(expression: &str) -> Vec<String> {
        Solution::brace_expansion_ii(expression.to_string())
    }

    #[test]
    fn example_concat_of_union_and_nested_union() {
        assert_eq!(
            expand("{a,b}{c,{d,e}}"),
            vec!["ac", "ad", "ae", "bc", "bd", "be"]
        );
    }

    #[test]
    fn example_nested_unions_keep_each_word_once() {
        assert_eq!(expand("{{a,z},a{b,c},{ab,z}}"), vec!["a", "ab", "ac", "z"]);
    }

    #[test]
    fn single_letter() {
        assert_eq!(expand("w"), vec!["w"]);
    }

    #[test]
    fn plain_word() {
        assert_eq!(expand("abc"), vec!["abc"]);
    }

    #[test]
    fn plain_union() {
        assert_eq!(expand("{a,b,c}"), vec!["a", "b", "c"]);
    }

    #[test]
    fn cartesian_product() {
        assert_eq!(expand("{a,b}{c,d}"), vec!["ac", "ad", "bc", "bd"]);
    }

    #[test]
    fn letters_wrapped_around_groups() {
        assert_eq!(
            expand("a{b,c}{d,e}f{g,h}"),
            vec![
                "abdfg", "abdfh", "abefg", "abefh", "acdfg", "acdfh", "acefg", "acefh",
            ]
        );
    }

    #[test]
    fn union_of_unions_collapses_duplicates() {
        assert_eq!(expand("{{a,b},{b,c}}"), vec!["a", "b", "c"]);
    }

    #[test]
    fn different_splits_of_the_same_word_are_merged() {
        // "ab"+"c" and "a"+"bc" both produce "abc".
        assert_eq!(expand("{ab,a}{c,bc}"), vec!["abbc", "abc", "ac"]);
    }

    #[test]
    fn twelve_binary_groups_cover_every_word_once() {
        let words = expand(&"{a,b}".repeat(12));
        assert_eq!(words.len(), 4096);
        assert!(words.is_sorted());
        assert!(words.windows(2).all(|pair| pair[0] != pair[1]));
        assert_eq!(words[0], "a".repeat(12));
        assert_eq!(words[4095], "b".repeat(12));
        assert!(words.iter().any(|word| word == "aaaabbbbbaab"));
    }
}
