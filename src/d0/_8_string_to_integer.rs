/*Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer

# Read in and ignore any leading whitespace.

# Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.

# Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.

# Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).

# If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.

# Return the integer as the final result.

Note:
- Only the space character ' ' is considered a whitespace character.
- Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
 */
struct Solution;
use std::i32;

impl Solution {
    fn my_atoi(s: String) -> i32 {
        println!("--------== _8_string_to_integer. s:{}", s);
        let mut str = s.trim_start();
        let mut out: i32 = 0;
        let mut positive = true;
        if str.len() > 1 {
            let c = &str[0..1];
            println!("c:{}", c);
            match c {
                "+" => {
                    str = &str[1..];
                }
                "-" => {
                    str = &str[1..];
                    positive = false;
                }
                _ => {
                    if let Some(c) = c.chars().next() {
                        if !('0'..='9').contains(&c) {
                            return 0;
                        }
                    }
                }
            }
        }

        for c in str.chars() {
            if ('0'..='9').contains(&c) {
                out = match out.checked_mul(10) {
                    None => {
                        return Self::get_max(positive);
                    }
                    Some(val) => val,
                };
                out = match out.checked_add((c as u8 - b'0') as i32) {
                    None => {
                        return Self::get_max(positive);
                    }
                    Some(val) => val,
                };
            } else {
                break;
            }
        }
        if !positive {
            out = match out.checked_mul(-1) {
                None => {
                    return Self::get_max(positive);
                }
                Some(val) => val,
            };
        }
        out
    }

    fn get_max(positive: bool) -> i32 {
        if positive {
            i32::MAX
        } else {
            i32::MIN
        }
    }
}

#[test]
fn test8() {
    let s = "42".to_string();
    let expected = 42;
    assert_eq!(Solution::my_atoi(s), expected);
    let s = "   -42".to_string();
    let expected = -42;
    assert_eq!(Solution::my_atoi(s), expected);
    let s = "4193 with words".to_string();
    let expected = 4193;
    assert_eq!(Solution::my_atoi(s), expected);
    let s = "words and 987".to_string();
    let expected = 0;
    assert_eq!(Solution::my_atoi(s), expected);
    let s = "-91283472332".to_string();
    let expected = -2_147_483_648;
    assert_eq!(Solution::my_atoi(s), expected);
}
