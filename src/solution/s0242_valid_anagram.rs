// Problem 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_count = HashMap::new();
    let mut t_count = HashMap::new();
    for (s_char, t_char) in s.chars().zip(t.chars()) {
        *s_count.entry(s_char).or_insert(0) += 1;
        *t_count.entry(t_char).or_insert(0) += 1;
    }

    s_count == t_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        let a = "anagram";
        let b = "nagaram";
        assert_eq!(is_anagram(a, b), true);

        let a = "rat";
        let b = "car";
        assert_eq!(is_anagram(a, b), false);
    }
}
