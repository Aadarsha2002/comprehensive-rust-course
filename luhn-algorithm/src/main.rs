// // TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut digits_seen = 0;
    for (i, ch) in cc_number.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        match ch.to_digit(10) {
            Some(digit) => {
                sum += if i % 2 == 0 {
                    digit
                } else {
                    let doubled = digit * 2;
                    doubled - if doubled > 9 { 9 } else { 0 }
                };
                digits_seen += 1;
            }
            None => return false,
        }
    }

    if digits_seen <= 2 {
        return false;
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    let cc_number = "1234 5678 1234 5670";

    println!(
        "Is {} a valid credit card number? {}",
        cc_number,
        match luhn(cc_number) {
            true => "Yes",
            false => "No",
        }
    );
}
