use std::collections::HashSet;

pub fn two_sum(nums: &Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
    let mut seen = HashSet::new();
    let mut j = i + 1;
    while j < nums.len() {
        let complement = -nums[i] - nums[j];
        if seen.contains(&complement) {
            res.push(vec![nums[i], nums[j], complement]);
            while j + 1 < nums.len() && nums[j] == nums[j + 1] {
                j += 1;
            }
        }
        seen.insert(nums[j]);
        j += 1;
    }
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted = nums.clone();
    sorted.sort();

    let mut results = vec![];
    for i in 0..sorted.len() {
        if sorted[i] > 0 {
            break; // Remaining values cannot sum to zero
        }
        if i == 0 || sorted[i - 1] != sorted[i] {
            // Skip repeated elements with 2nd conditional clause
            two_sum(&sorted, i, results.as_mut());
        }
    }

    results
}

#[cfg(test)]
#[test]
fn test_0() {
    assert_eq!(vec![-1, -1, 2], vec![-1, -1, 2]);
}

#[test]
fn test_1() {
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, 1, 0], vec![-1, 2, -1]]
    );
}

#[test]
fn test_2() {
    assert_eq!(three_sum(vec![]), Vec::<Vec<i32>>::new());
}

#[test]
fn test_3() {
    assert_eq!(three_sum(vec![0]), Vec::<Vec<i32>>::new());
}

#[test]
fn test_4() {
    assert_eq!(three_sum(vec![0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}
