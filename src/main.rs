mod solutions;
use Vec;
fn main() {
    solutions::solution_1::two_sum(Vec::from([1, 2, 3, 4, 5]), 5);
    solutions::solution_1119::remove_vowels(String::from("leetcodeisacommunityforcoders"));
    solutions::solution_2011::final_value_after_operations(vec![
        "++X".to_string(),
        "X++".to_string(),
        "--X".to_string(),
    ]);
    solutions::solution_2114::most_words_found(vec![
        "alice and bob love leetcode".to_string(),
        "i think so too".to_string(),
        "this is great thanks very much".to_string(),
    ]);
    solutions::solution_2114_functional::most_words_found(vec![
        "alice and bob love leetcode".to_string(),
        "i think so too".to_string(),
        "this is great thanks very much".to_string(),
    ]);
}
