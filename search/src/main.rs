#![allow(dead_code)]

use words_in_text::naive_find_word_in_text;

mod bfs;
mod bfs_maze;
mod binary_search;
mod dfs;
mod dfs_matrix_bad;
mod dfs_matrix_good;
mod words_in_text;

fn compute_bad_char_table(word: &str) -> [usize; 256] {
    let mut bad_char: [usize; 256] = [0; 256];
    for (i, char) in word.chars().enumerate() {
        bad_char[char as usize] = i + 1;
    }
    return bad_char;
}

fn boyer_moore_find_word_in_text(text: &str, word: &str) -> Vec<usize> {
    let bad_char = compute_bad_char_table(word);

    let text_len = text.len();
    let word_len = word.len();

    let mut indexes = Vec::new();
    let mut i = 0;
    while i <= text_len - word_len {
        let mut matching = true;
        for j in (0..word_len).rev() {
            let text_char = text.chars().nth(i + j).unwrap();
            let word_char = word.chars().nth(j).unwrap();
            if text_char != word_char {
                let bad_char_jump = bad_char[text_char as usize];
                let jump = j + 1 - bad_char_jump;
                matching = false;
                i += jump;
                break;
            }
        }
        if matching {
            indexes.push(i);
            i += 1;
        }
    }
    return indexes;
}

fn main() {
    // let text = "this world is a whole new world";
    // let occurences = naive_find_word_in_text(text, "world");
    // println!("occurences: {occurences:?}");

    let text = "this world is a whole new world";
    let occurences = boyer_moore_find_word_in_text(text, "world");
    println!("occurences: {occurences:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_find_word_in_text() {
        let text = "this world is a whole new world";
        let occurences = naive_find_word_in_text(text, "world");
        assert_eq!(occurences, vec![5, 26]);
    }

    #[test]
    fn test_boyer_moore_find_word_in_text() {
        let text = "this world is a whole new world";
        let occurences = boyer_moore_find_word_in_text(text, "world");
        assert_eq!(occurences, vec![5, 26]);
    }
}
