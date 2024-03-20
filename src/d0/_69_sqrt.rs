/*Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator.
 */
struct Solution;

impl Solution {
    fn my_sqrt(x: i32) -> i32 {
        println!("--------== 69 my_sqrt x= {:?}", x);
        (x as f64).sqrt().floor() as i32
    }
}

#[test]
fn test69() {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(9), 3);
}
