use std::collections::HashMap;

pub fn count_substrings(s: String) -> i32 {
    if s.len() <= 0 {
        return 0;
    }

    let mut ans = 0;
    let mut palindromes: HashMap<&str, bool> = HashMap::new();

    // Base case 1: single letter substrings
    // e.g. 'a', 'b'
    for i in 0..s.len() {
        palindromes.insert(&s[i..i + 1], true);
        ans += 1;
    }

    // Base case 2: 2-letter substrings
    // e.g. 'aa', 'bb'
    for i in 0..s.len() - 1 {
        let letter_1 = s.chars().nth(i).unwrap();
        let letter_2 = s.chars().nth(i + 1).unwrap();
        let chars_eq = letter_1 == letter_2;
        if chars_eq {
            ans += 1;
        }
        palindromes.insert(&s[i..i + 2], chars_eq);
    }

    // All other cases: substrings of length 3 to s.len()
    for len in 3..=s.len() {
        let mut i = 0;
        let mut j = len - 1;

        while j < s.len() {
            let first_char = s.chars().nth(i).unwrap();
            let last_char = s.chars().nth(j).unwrap();
            let substructure_result = palindromes.get(&s[i + 1..j]).unwrap();
            let is_palindrome =
                *substructure_result && (first_char == last_char);
            if is_palindrome {
                ans += 1;
            }
            palindromes.insert(&s[i..j + 1], is_palindrome);

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
