pub fn get_max_len(s: &str, mut left: i32, mut right: i32) -> i32 {
    let length = s.len() as i32;

    while left >= 0
        && right < length
        && &s[(left as usize)..(left + 1) as usize]
            == &s[(right as usize)..(right + 1) as usize]
    {
        left -= 1;
        right += 1;
    }

    return right - left - 1;
}

pub fn longest_palindrome(s: String) -> String {
    if s.len() == 0 {
        return "".to_string();
    }

    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() as i32 {
        let len1 = get_max_len(&s, i, i);
        let len2 = get_max_len(&s, i, i + 1);
        let max_len = std::cmp::max(len1, len2);
        if max_len > end - start {
            start = i - (max_len - 1) / 2;
            end = i + max_len / 2;
        }
    }

    String::from(&s[start as usize..end as usize + 1])
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(longest_palindrome("babad".to_string()), "aba".to_string());
}

#[test]
fn test_2() {
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
}

#[test]
fn test_3() {
    assert_eq!(longest_palindrome("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string()), "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string());
}
