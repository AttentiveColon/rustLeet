#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut val = x;
    let mut eval1: Vec<i32> = Vec::new();
    let mut eval2: Vec<i32> = Vec::new();

    while val != 0 {
        eval1.push(val % 10);
        eval2.insert(0, val % 10);
        val /= 10;
    }

    if eval1 == eval2 {
        return true;
    }

    return false;
}

#[test]
fn is_palindrome_test() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
}
