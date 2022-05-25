use std::collections::HashSet;

pub fn count_substrings(s: String) -> i32 {
    if s.len() <= 0 {
        return 0;
    }

    let mut ans = 0;
    let mut palindromes: HashSet<&str> = HashSet::new();

    // Base case 1: single letter substrings
    // e.g. 'a', 'b'
    for i in 0..s.len() {
        palindromes.insert(&s[i..i + 1]);
        ans += 1;
    }

    // Base case 2: 2-letter substrings
    // e.g. 'aa', 'bb'
    for i in 0..s.len() - 1 {
        let letter_1 = s.chars().nth(i).unwrap();
        let letter_2 = s.chars().nth(i + 1).unwrap();
        if letter_1 == letter_2 {
            ans += 1;
            palindromes.insert(&s[i..i + 2]);
        }
    }

    // All other cases: substrings of length 3 to s.len()
    for len in 3..=s.len() {
        let mut i = 0;
        let mut j = len - 1;

        while j < s.len() {
            let first_char = s.chars().nth(i).unwrap();
            let last_char = s.chars().nth(j).unwrap();
            if palindromes.contains(&s[i + 1..j]) && (first_char == last_char) {
                ans += 1;
                palindromes.insert(&s[i..j + 1]);
            }

            i += 1;
            j += 1;
        }
    }

    // Return
    ans
}
#[cfg(test)]
#[test]
fn test_1() {
    // "a", "b", "c"
    assert_eq!(count_substrings("abc".to_string()), 3);
}

#[test]
fn test_2() {
    // "a", "a", "a", "aa", "aa", "aaa"
    assert_eq!(count_substrings("aaa".to_string()), 6);
}

#[test]
fn test_3() {
    // "f", "d", "s", "k", "l", "f"
    assert_eq!(count_substrings("fdsklf".to_string()), 6);
}

#[test]
fn test_4() {
    assert_eq!(count_substrings("".to_string()), 0);
}

#[test]
fn test_5() {
    assert_eq!(count_substrings("aaaaa".to_string()), 15);
}
