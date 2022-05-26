pub fn climb_from(i: i32, n: i32) -> i32 {
    if i > n {
        0
    } else if i == n {
        1
    } else {
        climb_from(i + 1, n) + climb_from(i + 2, n)
    }
}

pub fn climb_stairs(n: i32) -> i32 {
    climb_from(0, n)
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(climb_stairs(1), 1);
}

#[test]
fn test_2() {
    assert_eq!(climb_stairs(2), 2);
}

#[test]
fn test_3() {
    assert_eq!(climb_stairs(3), 3);
}

#[test]
fn test_4() {
    assert_eq!(climb_stairs(4), 5);
}

#[test]
fn test_5() {
    assert_eq!(climb_stairs(5), 8);
}

#[test]
fn test_6() {
    assert_eq!(climb_stairs(6), 13);
}
