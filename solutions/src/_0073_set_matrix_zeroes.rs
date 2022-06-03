pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    println!("before: {matrix:?}");
    let m = matrix.len();
    let n = matrix[0].len();
    println!("mxn = {m}x{n}");

    // A flag which stores if the first row needs to be zero'd out later
    let mut col_dirty = false;

    // Assert flags
    for row in 0..m {
        if matrix[row][0] == 0 {
            col_dirty = true;
        }

        // Set the col and row headers to be 0s as flags
        for col in 1..n {
            if matrix[row][col] == 0 {
                matrix[0][col] = 0;
                matrix[row][0] = 0;
            }
        }
    }

    // Fill zeros (ignore headers for now)
    for row in 1..m {
        for col in 1..n {
            if matrix[row][0] == 0 || matrix[0][col] == 0 {
                matrix[row][col] = 0;
            }
        }
    }

    // See if first row needs to be zeroed
    if matrix[0][0] == 0 {
        for i in 0..n {
            matrix[0][i] = 0;
        }
    }

    // See if first column needs to be zeroed
    if col_dirty {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut start = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    set_zeroes(&mut start);
    assert_eq!(start, expected);
}

#[test]
fn test_2() {
    let mut start = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    let expected = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
    set_zeroes(&mut start);
    assert_eq!(start, expected);
}
