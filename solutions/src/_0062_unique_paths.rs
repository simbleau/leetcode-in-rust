use std::collections::HashMap;

pub fn recurse(m: i32, n: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if m == 0 || n == 0 {
        return 1;
    } else {
        let left_key = m - 1;
        let above_key = n - 1;

        let answer = match memo.get(&(left_key, above_key)) {
            Some(answer) => answer,
            None => {
                let answer =
                    recurse(left_key, n, memo) + recurse(m, above_key, memo);
                memo.insert((left_key, above_key), answer);
                return answer;
            }
        };

        *answer
    }
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    // Each answer is the amount of paths above and to the left
    let mut memoization = HashMap::new();
    recurse(m - 1, n - 1, &mut memoization)
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(unique_paths(3, 7), 28);
}

#[test]
fn test_2() {
    assert_eq!(unique_paths(3, 2), 3);
}

#[test]
fn test_3() {
    assert_eq!(unique_paths(1, 1), 1);
}
