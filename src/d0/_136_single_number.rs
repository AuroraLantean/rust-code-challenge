use core::num;

/*Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

You must implement a solution with a linear runtime complexity and use only constant extra space.
*/
struct Solution;

impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        println!("--------== 136 single_number. nums:{:?}", nums);
        //Use bitwise XOR operation to cancel out duplicate elements. XOR of a number with itself is 0, and XOR of 0 with a number is the number itself. By XORing all elements in the array, the remaining value will be the single number.
        nums.iter().fold(0i32, |res, val| res ^ val)
    }
}

#[test]
fn test136() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![1]), 1);
}
