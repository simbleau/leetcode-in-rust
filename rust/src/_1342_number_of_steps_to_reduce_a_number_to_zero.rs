pub fn number_of_steps(num: i32) -> i32 {
    let mut num = num.clone();
    let mut steps = 0;
    while num > 0 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num -= 1;
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(number_of_steps(14), 6);
}

#[test]
fn test_2() {
    assert_eq!(number_of_steps(8), 4);
}

#[test]
fn test_3() {
    assert_eq!(number_of_steps(123), 12);
}
