pub fn trace(
    board: &Vec<Vec<char>>,
    word: &String,
    word_path: String,
    path: Vec<(usize, usize)>,
) -> bool {
    // Base case
    if &word_path == word {
        return true;
    }

    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let next_char: char = match word.chars().nth(word_path.len()) {
        Some(c) => c,
        None => return false,
    };

    let (x, y) = *path.last().unwrap();
    for (delta_x, delta_y) in directions {
        let x_prime =
            (x as isize + delta_x).clamp(0, board.len() as isize - 1) as usize;
        let y_prime = (y as isize + delta_y)
            .clamp(0, board[0].len() as isize - 1)
            as usize;
        if path.contains(&(x_prime, y_prime)) {
            continue; // We've visited this before
        }
        let c_prime = board[x_prime][y_prime];
        if c_prime == next_char {
            let mut word_path = word_path.clone();
            word_path.push(c_prime);
            let mut path = path.clone();
            path.push((x_prime, y_prime));
            if trace(board, word, word_path, path) {
                return true;
            }
        }
    }
    false
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();

    // Find all beginning chars
    let starting_char = word.chars().next().unwrap();
    let mut starting_locs = Vec::new();
    for row in 0..m {
        for col in 0..n {
            if board[row][col] == starting_char {
                starting_locs.push((row, col));
            }
        }
    }

    // Attempt to snake around the starting locations for the entire word
    for starting_loc in starting_locs {
        if trace(
            &board,
            &word,
            String::from(starting_char),
            vec![starting_loc],
        ) {
            return true;
        }
    }

    false
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
}

#[test]
fn test_2() {
    assert_eq!(
        exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ),
        true
    );
}

#[test]
fn test_3() {
    assert_eq!(
        exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ),
        false
    );
}

#[test]
fn test_4() {
    assert_eq!(exist(vec![vec!['a']], "ab".to_string()), false);
}
