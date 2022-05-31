pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals.clone();
    intervals.sort();

    let mut merged: Vec<Vec<i32>> = vec![];
    for interval in intervals.iter() {
        if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
            // No overlap
            merged.push(interval.clone());
        } else {
            // Overlap exists
            merged.last_mut().unwrap()[1] =
                std::cmp::max(merged.last().unwrap()[1], interval[1]);
        }
    }

    merged
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
}

#[test]
fn test_2() {
    assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
}
