use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let primes = HashMap::from([
        ('a', 2),
        ('b', 3),
        ('c', 5),
        ('d', 7),
        ('e', 11),
        ('f', 13),
        ('g', 17),
        ('h', 19),
        ('i', 23),
        ('j', 29),
        ('k', 31),
        ('l', 37),
        ('m', 41),
        ('n', 43),
        ('o', 47),
        ('p', 53),
        ('q', 59),
        ('r', 61),
        ('s', 67),
        ('t', 71),
        ('u', 73),
        ('v', 79),
        ('w', 83),
        ('x', 89),
        ('y', 97),
        ('z', 101),
    ]);

    let mut anagrams: HashMap<u128, Vec<String>> = HashMap::new();

    for word in strs {
        let mut product: u128 = 1;
        for c in word.chars() {
            product *= primes[&c];
        }
        match anagrams.get_mut(&product) {
            Some(matches) => matches.push(word.clone()),
            None => {
                anagrams.insert(product, vec![word.clone()]);
            }
        };
    }

    anagrams.into_values().collect()
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ]),
        vec![
            vec!["bat".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
        ]
    );
}

#[test]
fn test_2() {
    assert_eq!(
        group_anagrams(vec!["".to_string()]),
        vec![vec!["".to_string()]]
    );
}

#[test]
fn test_3() {
    assert_eq!(
        group_anagrams(vec!["a".to_string()]),
        vec![vec!["a".to_string()]]
    );
}

#[test]
fn test_4() {
    assert_eq!(
        group_anagrams(vec![
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
        ]),
        vec![
            vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()],
            vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()],
        ]
    );
}
