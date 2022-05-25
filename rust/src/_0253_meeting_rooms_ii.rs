pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    let max =
        *intervals.iter().map(|i| i.get(1).unwrap()).max().unwrap() as usize;
    let mut prefix_sum = vec![0; (max + 1) as usize];

    for interval in intervals {
        let meeting_start = interval.get(0).unwrap();
        let meeting_end = interval.get(1).unwrap();

        // Increment start with 1
        let start = prefix_sum.get_mut(*meeting_start as usize).unwrap();
        *start += 1;
        // Decrement end with -1
        let end = prefix_sum.get_mut(*meeting_end as usize).unwrap();
        *end -= 1;
    }

    let mut max_overlapping_meetings = 0;
    for i in 1..max {
        // Construct prefix sum
        let prev = *prefix_sum.get(i - 1).unwrap();
        let prefix_sum_i = prefix_sum.get_mut(i).unwrap();
        *prefix_sum_i += prev;
        max_overlapping_meetings =
            std::cmp::max(*prefix_sum_i, max_overlapping_meetings);
    }

    // Return
    max_overlapping_meetings
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        min_meeting_rooms(vec![vec![5, 10], vec![0, 30], vec![15, 20]]),
        2
    );
}

#[test]
fn test_2() {
    assert_eq!(min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]), 1);
}
