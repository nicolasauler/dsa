fn bitiling_dominoes_and_triminoes_down_up(cols_n: usize) -> usize {
    let mut size_states: Vec<Vec<usize>> = vec![vec![0; cols_n + 1]; 4];
    size_states[3][0] = 1;

    for _i in 1..cols_n {}

    return size_states[3][cols_n];
}

/// 2xcol board
/// can put __ 2x1 domino or |- (L-shaped tromino_
///                          |
///
/// to get f(4), you can either
/// add to f(2) two __
///                 __
///
/// or add to f(3) a |
///                  |
///
/// or add to a partial p(3) a tromino (on either way)
///
///
/// to get a partial p(2)
/// you have to either add a tromino to a f(0)
/// or add a __ to a p(1)
fn bitiling_dominoes_and_triminoes_pattern(cols_n: usize) -> usize {
    let mut full: Vec<usize> = vec![0; cols_n + 1];
    full[0] = 1;
    full[1] = 1;

    let mut partial: Vec<usize> = vec![0; cols_n + 1];
    // partial[2] = 1

    for col in 2..=cols_n {
        partial[col] = partial[col - 1] + full[col - 2];
        full[col] = full[col - 1] + full[col - 2] + 2 * partial[col - 1];
    }

    return full[cols_n];
}

fn bitiling_dominoes_and_triminoes_pattern_leetcode(n: i32) -> i32 {
    let n = n as usize;
    let mut full: Vec<u64> = vec![0; n + 1];
    full[0] = 1;
    full[1] = 1;

    let mut partial: Vec<u64> = vec![0; n + 1];
    // partial[2] = 1

    for col in 2..=n {
        partial[col] = (partial[col - 1] + full[col - 2]) % 1_000_000_007;
        full[col] = (full[col - 1] + full[col - 2] + 2 * partial[col - 1]) % 1_000_000_007;
    }

    return full[n] as i32;
}

fn bitiling_dominoes_and_triminoes_pattern_top_down_p(
    f_cache: &mut [Option<usize>],
    p_cache: &mut [Option<usize>],
    cols_n: usize,
) -> usize {
    if p_cache[cols_n].is_some() {
        return p_cache[cols_n].unwrap();
    }

    if cols_n == 0 {
        p_cache[0] = Some(0);
        return 0;
    }
    if cols_n == 1 {
        p_cache[1] = Some(0);
        return 0;
    }

    let val = bitiling_dominoes_and_triminoes_pattern_top_down_p(f_cache, p_cache, cols_n - 1)
        + bitiling_dominoes_and_triminoes_pattern_top_down_f(f_cache, p_cache, cols_n - 2);
    return val;
}

fn bitiling_dominoes_and_triminoes_pattern_top_down_f(
    f_cache: &mut [Option<usize>],
    p_cache: &mut [Option<usize>],
    cols_n: usize,
) -> usize {
    if f_cache[cols_n].is_some() {
        return f_cache[cols_n].unwrap();
    }

    if cols_n == 0 {
        f_cache[0] = Some(1);
        return 1;
    }
    if cols_n == 1 {
        f_cache[1] = Some(1);
        return 1;
    }

    let val = bitiling_dominoes_and_triminoes_pattern_top_down_f(f_cache, p_cache, cols_n - 1)
        + bitiling_dominoes_and_triminoes_pattern_top_down_f(f_cache, p_cache, cols_n - 2)
        + 2 * bitiling_dominoes_and_triminoes_pattern_top_down_p(f_cache, p_cache, cols_n - 1);
    return val;
}

fn bitiling_dominoes_and_triminoes_pattern_top_down(cols_n: usize) -> usize {
    let mut f_cache: Vec<Option<usize>> = vec![None; cols_n + 1];
    let mut p_cache: Vec<Option<usize>> = vec![None; cols_n + 1];

    return bitiling_dominoes_and_triminoes_pattern_top_down_f(&mut f_cache, &mut p_cache, cols_n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitiling_dominoes_and_triminoes_pattern() {
        let n_tilings = bitiling_dominoes_and_triminoes_pattern(2);
        assert_eq!(n_tilings, 2);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern(3);
        assert_eq!(n_tilings, 5);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern(4);
        assert_eq!(n_tilings, 11);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern(5);
        assert_eq!(n_tilings, 24);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern(30);
        assert_eq!(n_tilings, 9312342245);
    }

    #[test]
    fn test_bitiling_dominoes_and_triminoes_pattern_leetcode() {
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_leetcode(2);
        assert_eq!(n_tilings, 2);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_leetcode(3);
        assert_eq!(n_tilings, 5);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_leetcode(4);
        assert_eq!(n_tilings, 11);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_leetcode(5);
        assert_eq!(n_tilings, 24);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_leetcode(30);
        assert_eq!(n_tilings, 312342182);
    }

    #[test]
    fn test_bitiling_dominoes_and_triminoes_pattern_top_down() {
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_top_down(2);
        assert_eq!(n_tilings, 2);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_top_down(3);
        assert_eq!(n_tilings, 5);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_top_down(4);
        assert_eq!(n_tilings, 11);
        let n_tilings = bitiling_dominoes_and_triminoes_pattern_top_down(5);
        assert_eq!(n_tilings, 24);
        // TAKES A LOOOONG TIME
        //let n_tilings = bitiling_dominoes_and_triminoes_pattern_top_down(30);
        //assert_eq!(n_tilings, 9312342245);
    }
}
