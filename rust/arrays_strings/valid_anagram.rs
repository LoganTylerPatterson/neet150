use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            *char_map.entry(c).or_insert(0) -= 1;
        }

        for (_, count) in char_map {
            if count != 0 {
                return false;
            }
        }

        true
    }
}
