fn dfs_matrix(matrix: &[Vec<bool>], visited: &mut [Vec<bool>], at: (usize, usize)) {
    let (row, col) = at;
    if visited[row][col] {
        return;
    }
    visited[row][col] = true;
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let row_dir: [i32; 4] = [-1, 1, 0, 0];
    let col_dir: [i32; 4] = [0, 0, 1, -1];
    for i in 0..4 {
        let new_row = row as i32 + row_dir[i];
        let new_col = col as i32 + col_dir[i];
        if new_row >= 0
            && new_row < row_len as i32
            && new_col >= 0
            && new_col < col_len as i32
            && matrix[new_row as usize][new_col as usize]
        {
            dfs_matrix(matrix, visited, (new_row as usize, new_col as usize));
        }
    }
}

fn remove_islands(matrix: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; col_len]; row_len];

    for row in 0..row_len {
        for col in 0..col_len {
            if !visited[row][col]
                && matrix[row][col]
                && (row % row_len == 0
                    || row % row_len == row_len - 1
                    || col % col_len == 0
                    || col % col_len == col_len - 1)
            {
                dfs_matrix(&matrix, &mut visited, (row, col));
            }
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_islands() {
        let matrix: Vec<Vec<bool>> = vec![
            vec![true, false, false, false, false, false],
            vec![false, true, false, true, true, true],
            vec![false, false, true, false, true, false],
            vec![true, true, false, false, true, false],
            vec![true, false, true, true, false, false],
            vec![true, false, false, false, false, true],
        ];

        let result = remove_islands(&matrix);

        let expected: Vec<Vec<bool>> = vec![
            vec![true, false, false, false, false, false],
            vec![false, false, false, true, true, true],
            vec![false, false, false, false, true, false],
            vec![true, true, false, false, true, false],
            vec![true, false, false, false, false, false],
            vec![true, false, false, false, false, true],
        ];
        assert_eq!(result, expected);
    }
}
