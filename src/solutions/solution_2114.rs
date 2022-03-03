use std::cmp;

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let mut max_word_count = i32::MIN;
    for sentence in sentences.iter() {
        let split_words: Vec<&str> = sentence.split_whitespace().collect();
        max_word_count = cmp::max(max_word_count, split_words.len() as i32);
    }
    max_word_count
}
