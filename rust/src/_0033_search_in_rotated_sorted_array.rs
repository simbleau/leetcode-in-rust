pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;

    while start <= end {
        let mid = start + (end - start) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] >= nums[start] {
            // Left half is not rotated
            if target >= nums[start] && target < nums[mid] {
                // Target is located in the first half, move towards
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        } else {
            // Right half is not rotated
            if target > nums[mid] && target <= nums[end] {
                // Target is located in the right half, move towards
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
    }
    -1
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
}

#[test]
fn test_2() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}

#[test]
fn test_3() {
    assert_eq!(search(vec![1], 0), -1);
}

#[test]
fn test_4() {
    assert_eq![search(vec![5, 1, 3], 5), 0];
}

#[test]
fn test_5() {
    assert_eq!(search(vec![5, 1, 3], 3), 2)
}
