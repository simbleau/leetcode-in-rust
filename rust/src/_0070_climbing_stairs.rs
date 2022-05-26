use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn climb_from(
    i: i32,
    n: i32,
    memoization: Rc<RefCell<HashMap<i32, i32>>>,
) -> i32 {
    if i > n {
        0
    } else if i == n {
        1
    } else if memoization.as_ref().borrow().contains_key(&i) {
        return *memoization.as_ref().borrow().get(&i).unwrap();
    } else {
        let v1 = climb_from(i + 1, n, memoization.clone());
        let v2 = climb_from(i + 2, n, memoization.clone());
        memoization.as_ref().borrow_mut().insert(i, v1 + v2);
        *memoization.as_ref().borrow().get(&i).unwrap()
    }
}

pub fn climb_stairs(n: i32) -> i32 {
    let memoization = Rc::new(RefCell::new(HashMap::new()));
    climb_from(0, n, memoization)
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
