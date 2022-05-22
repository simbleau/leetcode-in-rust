pub fn max_area(height: Vec<i32>) -> i32 {
    let mut p1 = 0;
    let mut p2 = height.len() - 1;

    let mut max_height = 0;
    while p1 < p2 && p2 > p1 {
        // Determine limiting height
        let h1 = height.get(p1).unwrap();
        let h2 = height.get(p2).unwrap();
        let curr_height = std::cmp::min(h1, h2) * (p2 - p1) as i32;
        max_height = std::cmp::max(curr_height, max_height);

        // Move one of the pointers
        if h1 < h2 {
            p1 += 1;
        } else {
            p2 -= 1;
        }
    }

    max_height
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}

#[test]
fn test_2() {
    assert_eq!(max_area(vec![1, 1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
}
