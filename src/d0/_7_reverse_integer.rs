/*Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 */
struct Solution;

impl Solution {
    fn reverse(x: i32) -> i32 {
        println!("--------== reverse. x:{}", x);
        let x_str = x.abs().to_string().chars().rev().collect::<String>();

        if let Ok(y) = x_str.parse::<i32>() {
            x.signum() * y
        } else {
            0
        }
    }
}

#[test]
fn test7() {
    let x = 2_147_483_647;
    let res = 0;
    assert_eq!(Solution::reverse(x), res);
    let x = 123_456_789;
    let res = 987_654_321;
    assert_eq!(Solution::reverse(x), res);
    let x = -123;
    let res = -321;
    assert_eq!(Solution::reverse(x), res);
}
