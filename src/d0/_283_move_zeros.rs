use core::num;

/*Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array. */
struct Solution;

impl Solution {
    fn move_zeroes(nums: &mut Vec<i32>) {
        println!("--------== 283 move_zeroes. nums = {:?}", nums);
        let mut j = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            if x != 0 {
                nums[i] = 0;
                nums[j] = x;
                j += 1;
            }
        }
    }
}

#[test]
fn test283() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0]);
}
