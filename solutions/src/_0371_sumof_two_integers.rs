pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut x = a.abs();
    let mut y = b.abs();

    // Ensure abs(a) > abs(b)
    if x < y {
        return get_sum(b, a);
    }

    let sign = match a > 0 {
        true => 1,
        false => -1,
    };

    if a * b >= 0 {
        // Sum of two positive integers, x + y, where x > y
        while y != 0 {
            let answer = x ^ y; // XOR the numbers
            let carry = (x & y) << 1; // Determine the carry
            x = answer;
            y = carry;
        }
        x * sign
    } else {
        // Difference of two integer, x - y, where x > y
        while y != 0 {
            let answer = x ^ y; // XOR the numbers
            let borrow = ((!x) & y) << 1; // Determine the carry
            x = answer;
            y = borrow;
        }
        x * sign
    }
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(get_sum(1, 1), (2));
}

#[test]
fn test_2() {
    assert_eq!(get_sum(5, 8), (13));
}

#[test]
fn test_3() {
    assert_eq!(get_sum(5, -8), (-3));
}

#[test]
fn test_4() {
    assert_eq!(get_sum(-12, -8), (-20));
}
