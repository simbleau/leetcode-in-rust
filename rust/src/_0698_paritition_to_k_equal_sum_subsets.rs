pub fn backtrack(
    nums: &Vec<i32>,            // The list of numbers
    taken_nums: &mut Vec<bool>, // A map of used numbers during this iteration
    target_sum: i32,            // The target sum for a subarray
    k: i32,                     // The amount of subsets
    index: usize,               // The current index for iteration
    count: i32,                 // Current track of counted subarrays
    cur_sum: i32,               // Current track of sum
) -> bool {
    let n = nums.len();

    // The correct amount of subarrays has been made
    if count == k - 1 {
        return true;
    }

    // If current sum is greater, this path is bust.
    if cur_sum > target_sum {
        return false;
    }

    // A subset is made. Check if others exist, continue.
    if cur_sum == target_sum {
        return backtrack(nums, taken_nums, target_sum, k, 0, count + 1, 0);
    }

    // Try elements not picked already to make combinations
    for i in index..n {
        // Take the number
        if !taken_nums[i] {
            taken_nums[i] = true;

            // Test if it can make an element
            if backtrack(
                nums,
                taken_nums,
                target_sum,
                k,
                i + 1,
                count,
                cur_sum + nums[i],
            ) {
                return true;
            }

            // Reverse the take and try again
            taken_nums[i] = false;
        }
    }

    // Unable to make a valid combination after picking every element from the
    // array, and hence, we cannot make k subsets.
    false
}

pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();

    let total: i32 = nums.iter().sum();
    if total % k != 0 {
        // Total is not divisible by the amount of subsets requested
        return false;
    }
    let target_sum = total / k;

    // Sort in reverse for optimization
    let mut nums = nums.clone();
    nums.sort();
    nums.reverse();

    let mut taken_nums = vec![false; n];
    backtrack(&nums, &mut taken_nums, target_sum, k, 0, 0, 0)
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
}

#[test]
fn test_2() {
    assert_eq!(can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
}

#[test]
fn test_3() {
    assert_eq!(can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4), false);
}

#[test]
fn test_4() {
    assert_eq!(
        can_partition_k_subsets(
            vec![
                18, 20, 39, 73, 96, 99, 101, 111, 114, 190, 207, 295, 471, 649,
                700, 1037
            ],
            4
        ),
        true
    );
}
