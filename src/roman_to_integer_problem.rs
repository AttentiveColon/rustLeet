#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let nums: Vec<i32> = s
        .chars()
        .rev()
        .map(|c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                panic!("wtf");
            }
        })
        .collect();

    let mut sum = 0;
    let mut prev_num = 0;

    for num in nums {
        if num < prev_num {
            sum -= num;
        } else {
            sum += num;
        }
        prev_num = num;
    }

    sum
}

#[test]
fn roman_to_int_test() {
    assert_eq!(roman_to_int("III".to_owned()), 3);
    assert_eq!(roman_to_int("LVIII".to_owned()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_owned()), 1994);
}
