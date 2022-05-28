// Template for a solution
pub fn sum_zero(n: i32) -> Vec<i32> {
    let mut buffer = vec![0; n as usize];
    if n < 2 {
        return buffer;
    }

    let half = n / 2;
    if n % 2 == 1 {
        let half_plus_one = half + 1;
        for i in 0..half {
            unsafe {
                // Safety: Items must exist
                let item1 = buffer.get_unchecked_mut(i as usize);
                *item1 -= i + 1;
                let item2 =
                    buffer.get_unchecked_mut((half_plus_one + i) as usize);
                *item2 += i + 1;
            }
        }
    } else {
        for i in 0..half {
            unsafe {
                // Safety: Items must exist
                let item1 = buffer.get_unchecked_mut(i as usize);
                *item1 -= i + 1;
                let item2 = buffer.get_unchecked_mut((half + i) as usize);
                *item2 += i + 1;
            }
        }
    }
    buffer
}
#[cfg(test)]
#[test]
fn test_1() {
    let n = 5;
    let answer = sum_zero(n);
    assert!(answer.len() == n as usize);
    assert!(answer.iter().sum::<i32>() == 0);
    // Test duplicates
    for (i, n) in answer.iter().enumerate() {
        for (j, n2) in answer.iter().enumerate() {
            assert!(n != n2 || i == j);
        }
    }
}

#[test]
fn test_2() {
    let n = 3;
    let answer = sum_zero(n);
    assert!(answer.len() == n as usize);
    assert!(answer.iter().sum::<i32>() == 0);
    // Test duplicates
    for (i, n) in answer.iter().enumerate() {
        for (j, n2) in answer.iter().enumerate() {
            assert!(n != n2 || i == j);
        }
    }
}

#[test]
fn test_3() {
    let n = 1;
    let answer = sum_zero(n);
    assert!(answer.len() == n as usize);
    assert!(answer.iter().sum::<i32>() == 0);
    // Test duplicates
    for (i, n) in answer.iter().enumerate() {
        for (j, n2) in answer.iter().enumerate() {
            assert!(n != n2 || i == j);
        }
    }
}

#[test]
fn test_4() {
    let n = 4;
    let answer = sum_zero(n);
    assert!(answer.len() == n as usize);
    assert!(answer.iter().sum::<i32>() == 0);
    // Test duplicates
    for (i, n) in answer.iter().enumerate() {
        for (j, n2) in answer.iter().enumerate() {
            assert!(n != n2 || i == j);
        }
    }
}
