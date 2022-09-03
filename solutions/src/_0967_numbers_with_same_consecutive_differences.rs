use std::collections::HashMap;

fn is_same_consec_diff(
    mut num: String,
    memo: &mut HashMap<String, bool>,
    k: i32,
) -> bool {
    let digits = num.len() as i32;
    let mut chars = num.chars();
    let d1 = chars.next_back().unwrap().to_digit(10).unwrap() as i32;
    let d2 = chars.next_back().unwrap().to_digit(10).unwrap() as i32;
    let result = if digits <= 2 {
        (d1 - d2).abs() == k
    } else {
        num.pop().unwrap();
        (d1 - d2).abs() == k && is_same_consec_diff(num.clone(), memo, k)
    };
    memo.insert(num, result);
    result
}

pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    let mut memo = HashMap::new();
    let mut nums = vec![];

    let start_inclusive = 10_i32.pow((n - 1) as u32);
    let end_exclusive = 10_i32.pow(n as u32);
    for i in start_inclusive..end_exclusive {
        if is_same_consec_diff(i.to_string(), &mut memo, k) {
            nums.push(i);
        }
    }

    nums
}

#[cfg(test)]
#[test]
fn test_recursive() {
    let mut memo = HashMap::new();
    assert_eq!(
        is_same_consec_diff("1313535".to_string(), &mut memo, 2),
        true
    );
}

#[test]
fn test_1() {
    assert_eq!(nums_same_consec_diff(3, 7), vec![181, 292, 707, 818, 929]);
}

#[test]
fn test_2() {
    assert_eq!(
        nums_same_consec_diff(2, 1),
        vec![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98
        ]
    );
}
