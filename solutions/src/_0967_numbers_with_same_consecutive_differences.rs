fn dfs(results: &mut Vec<i32>, num: i64, min: i64, max: i64, k: i64) {
    let d1: i64 = (num % 100) / 10;
    let d2: i64 = num % 10;

    let result = (d1 - d2).abs() == k;
    if num >= min && num <= max && result {
        if !results.contains(&(num as i32)) {
            results.push(num as i32);
        }
    }

    if d2 >= k {
        let num = num * 10 + d2 - k;
        if num <= max {
            dfs(results, num, min, max, k);
        }
    }
    if d2 + k <= 9 {
        let num = num * 10 + d2 + k;
        if num <= max {
            dfs(results, num, min, max, k);
        }
    }
}

pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    let mut nums = vec![];

    let min = 10_i64.pow((n - 1) as u32);
    let max = 10_i64.pow(n as u32) - 1;
    for i in 1..=9 {
        dfs(&mut nums, i, min, max, k as i64);
    }

    nums
}

#[cfg(test)]
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

#[test]
fn test_3() {
    assert_eq!(
        nums_same_consec_diff(2, 0),
        vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
    );
}

#[test]
fn test_4() {
    assert_eq!(
        nums_same_consec_diff(9, 0),
        vec![
            111111111, 222222222, 333333333, 444444444, 555555555, 666666666,
            777777777, 888888888, 999999999
        ]
    );
}
