pub fn num_decodings(s: String) -> i32 {
    let mut memo = vec![0; s.len() + 1];
    memo[0] = 1;

    // Ways to decode a string of size 1 is 1 unless first char is 0
    if &s[0..1] == "0" {
        memo[1] = 0;
    } else {
        memo[1] = 1;
    }

    for i in 2..memo.len() {
        // Check if single digit decode is possible
        if &s[i - 1..i] != "0" {
            memo[i] = memo[i - 1]
        }

        // Check if single digit decode is possible
        let two_digit = s[i - 2..i].parse::<i32>().unwrap();
        if two_digit >= 10 && two_digit <= 26 {
            memo[i] += memo[i - 2];
        }
    }

    memo[s.len()]
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(num_decodings("12".to_string()), 2);
}

#[test]
fn test_2() {
    assert_eq!(num_decodings("226".to_string()), 3);
}

#[test]
fn test_3() {
    assert_eq!(num_decodings("06".to_string()), 0);
}
