#![allow(dead_code)]
mod avl_tree;
mod btree;
mod linked_list;
mod trie;

fn build_suffix_array() -> Vec<usize> {
}

fn is_substring_bsearch(suffix_array: &[usize], text: &str, query: &str) -> bool {
    let mut left: usize = 0;
    let mut right: usize = suffix_array.len() - 1;
    let mut middle: usize;

    while left <= right {
        middle = (left + right) / 2;
        let substring = &text[suffix_array[middle]..];
        match substring.cmp(&query) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => left = middle + 1,
            std::cmp::Ordering::Greater => right = middle.saturating_sub(1),
        }
    }

    false
}

fn main() {
    let text = String::from("abracadabradad");
    let suffix_array = [14, 0, 7, 3, 12, 5, 10, 1, 8, 4, 13, 6, 11, 2, 9];
    let query = String::from("dad");

    let is_substring = is_substring_bsearch(&suffix_array, &text, &query);
    println!("Is substring: {}", is_substring);
}
