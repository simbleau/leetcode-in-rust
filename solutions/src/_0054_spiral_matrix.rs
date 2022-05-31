pub fn spiral(
    dir: i8,
    matrix: &Vec<Vec<i32>>,
    buf: &mut Vec<i32>,
    startx: usize,
    starty: usize,
    endx: usize,
    endy: usize,
) {
    match dir {
        0 => {
            // Right
            for i in startx..endx {
                buf.push(matrix[starty][i]);
            }
            if endx - startx > 0 {
                spiral(1, matrix, buf, startx, starty + 1, endx, endy);
            }
        }
        1 => {
            // Down
            for i in starty..endy {
                buf.push(matrix[i][endx - 1]);
            }
            if endy - starty > 0 {
                spiral(2, matrix, buf, startx, starty, endx - 1, endy);
            }
        }
        2 => {
            // Left
            for i in (startx..endx).rev() {
                buf.push(matrix[endy - 1][i]);
            }
            if endx - startx > 0 {
                spiral(3, matrix, buf, startx, starty, endx, endy - 1);
            }
        }
        _ => {
            // Up
            for i in (starty..endy).rev() {
                buf.push(matrix[i][startx]);
            }
            if endy - starty > 0 {
                spiral(0, matrix, buf, startx + 1, starty, endx, endy);
            }
        }
    }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let m = match n {
        0 => 0,
        _ => matrix[0].len(),
    };
    let mut buf = vec![];
    spiral(0, &matrix, &mut buf, 0, 0, m, n);
    buf
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        [1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
}

#[test]
fn test_2() {
    assert_eq!(
        spiral_order(vec![vec![2, 5, 8], vec![4, 0, -1]]),
        [2, 5, 8, -1, 0, 4]
    );
}

#[test]
fn test_3() {
    assert_eq!(spiral_order(vec![vec![0], vec![1]]), [0, 1]);
}
