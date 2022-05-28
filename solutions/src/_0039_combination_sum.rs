pub fn backtrack(
    candidates: &Vec<i32>,
    start: usize,
    combination: &mut Vec<i32>,
    remaining: i32,
    results: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        results.push(combination.clone());
        return;
    } else if remaining < 0 {
        return;
    }

    for i in start..candidates.len() {
        let num = candidates[i];
        combination.push(num);
        backtrack(candidates, i, combination, remaining - num, results);
        combination.pop();
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut results = vec![];
    backtrack(&candidates, 0, &mut vec![], target, &mut results);
    results
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
}
