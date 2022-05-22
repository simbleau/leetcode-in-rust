use std::collections::HashSet;

pub fn max_length(arr: Vec<String>) -> i32 {
    // Start with an empty string in the results array
    // to build future results
    let mut results: Vec<HashSet<char>> = vec![HashSet::new()];
    let mut max = 0;

    for word in arr.iter() {
        // A buffer for results since we cannot modify the results iterating
        let mut result_buffer = vec![];

        // Iterate through results which existed prior to this loop
        for char_set in results.iter() {
            let mut new_char_set = char_set.clone();
            new_char_set.extend(word.chars());
            // Get the amount of characters added to the hashset
            let chars_added = new_char_set.len() - char_set.len();
            // Were all the added characters unique?
            let unique = chars_added == word.len();
            if !unique {
                // Characters were repeated when adding to the previous, or the
                // word itself was not unique
                // e.g. 'abc' + 'cde' = 'abcde'
                // e.g. 'aa' + 'bb' = 'ab'
                continue;
            }
            if new_char_set.len() > max {
                max = new_char_set.len();
            }
            result_buffer.push(new_char_set);
        }
        results.extend(result_buffer);
    }

    max as i32
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]),
        4
    );
}

#[test]
fn test_2() {
    assert_eq!(
        max_length(vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string()
        ]),
        6
    );
}

#[test]
fn test_3() {
    assert_eq!(
        max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]),
        26
    );
}

#[test]
fn test_4() {
    assert_eq!(max_length(vec!["aa".to_string(), "bb".to_string()]), 0);
}

#[test]
fn test_6() {
    assert_eq!(
        max_length(vec![
            "a".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "de".to_string(),
            "def".to_string()
        ]),
        6
    );
}
