use std::collections::BinaryHeap; // Priority Queue
use std::collections::HashMap; // Map

pub fn min_deletions(s: String) -> i32 {
    // Store the frequencies of every character
    let mut map = HashMap::new();
    for c in s.chars() {
        match map.get_mut(&c) {
            Some(amt) => {
                *amt += 1;
            }
            None => {
                map.insert(c, 1);
            }
        };
    }

    // Create a priority_queue, say pq to store the frequency of each character
    // such that the largest frequency obtained is present at the top of the
    // priority queue pq.
    let mut pq = BinaryHeap::new();
    for (_, amt) in map {
        pq.push(amt);
    }

    // Iterate over the priority_queue until pq is empty and check if the
    // topmost of element of pq is equal to the second topmost element of pq or
    // not. If found to be true, then decrement the value of topmost element of
    // pq by 1 and increment the value of removals by 1.
    let mut removals = 0;
    while !pq.is_empty() {
        let top_element: i32 = pq.pop().unwrap();
        if pq.is_empty() {
            return removals;
        }

        // Safety: not empty
        let top_2nd_element = pq.peek().unwrap();
        if top_element == *top_2nd_element {
            if top_element > 1 {
                pq.push(top_element - 1);
            }
            removals += 1;
        }
    }

    removals
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(min_deletions("aab".to_string()), 0);
}
#[test]
fn test_2() {
    assert_eq!(min_deletions("aaabbbcc".to_string()), 2);
}
#[test]
fn test_3() {
    assert_eq!(min_deletions("ceabaacb".to_string()), 2);
}

#[test]
fn test_4() {
    assert_eq!(min_deletions("bbcebab".to_string()), 2);
}
