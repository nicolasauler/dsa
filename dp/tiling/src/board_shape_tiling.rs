/// types of pieces:
/// - size 1 color 1
/// - size 1 color 2
/// - size 2
/// - size 4
fn n_tilings_for_board(table: &mut [usize], size: usize) -> usize {
    if size == 0 {
        return 1;
    }
    if size == 1 {
        return 2;
    }

    table[0] = 1;
    table[1] = 2;

    for i in 2..=size {
        if i > 3 {
            table[i] = 2 * table[i - 1] + table[i - 2] + table[i - 4];
        } else {
            table[i] = 2 * table[i - 1] + table[i - 2];
        }
    }

    return table[size];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_tilings_for_board() {
        let mut saved: Vec<usize> = vec![0; 26];
        let n_tilings = n_tilings_for_board(&mut saved, 4);
        assert_eq!(n_tilings, 30);
    }
}
