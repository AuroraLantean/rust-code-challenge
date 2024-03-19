/*Given an integer x, return true if x is a
palindrome, and false otherwise.
*/
struct Solution;

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        println!("--------== is_palindrome. x:{}", x);
        if x < 0 {
            return false;
        }
        let mut reversed = 0;
        let mut num = x;
        while num > 0 {
            println!("reversed: {}, num:{}", reversed, num);
            reversed = reversed * 10 + num % 10;
            println!("reversed: {}", reversed);
            num /= 10;
        }
        x == reversed
        // or compare original num as a string with the reversed string
        // let x_str = x.abs().to_string();
        // x >= 0 && x_str == x_str.chars().rev().collect::<String>()
    }
}

#[test]
fn test9() {
    let x = 121;
    let res = true;
    assert_eq!(Solution::is_palindrome(x), res);

    let x = 10;
    let res = false;
    assert_eq!(Solution::is_palindrome(x), res);

    let x = -121;
    let res = false;
    assert_eq!(Solution::is_palindrome(x), res);

    let x = 123;
    let res = false;
    assert_eq!(Solution::is_palindrome(x), res);

    let x = 12321;
    let res = true;
    assert_eq!(Solution::is_palindrome(x), res);
}
