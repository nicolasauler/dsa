pub fn naive_find_word_in_text(text: &str, word: &str) -> Vec<usize> {
    let mut indexes = Vec::new();

    for i in 0..text.len() {
        let mut matching = true;
        for j in 0..word.len() {
            if text[(i + j)..(i + j + 1)] != word[j..j + 1] {
                matching = false;
                break;
            }
        }
        if matching {
            indexes.push(i);
        }
    }

    return indexes;
}
