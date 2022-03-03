use std::collections::HashSet;

pub fn remove_vowels(s: String) -> String {
    let vowel_set: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut out = String::from("");
    for ch in s.chars() {
        if !vowel_set.contains(&ch) {
            out += &ch.to_string();
        }
    }
    out
}
