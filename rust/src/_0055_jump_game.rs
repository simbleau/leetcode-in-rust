pub fn can_jump(nums: Vec<i32>) -> bool {
    let goal = nums.len() - 1;

    // Memoization
    let mut cache = vec![false; nums.len()];
    *cache.get_mut(goal).unwrap() = true;

    // Dynamic programming
    let mut min_reachable_solution = goal;
    for i in (0..nums.len()).rev() {
        let max_reach = std::cmp::min(i + *nums.get(i).unwrap() as usize, goal);
        let can_i_reach = max_reach >= min_reachable_solution;
        if can_i_reach {
            *cache.get_mut(i).unwrap() = true;
            min_reachable_solution = i;
        }
    }

    *cache.get(0).unwrap()
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
}

#[test]
fn test_2() {
    assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
}

#[test]
fn test_3() {
    assert_eq!(can_jump(vec![1, 0, 1, 0]), false);
}

#[test]
fn test_4() {
    assert_eq!(can_jump(vec![1, 2, 3]), true);
}
