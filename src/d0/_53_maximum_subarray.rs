/*Given an integer array nums, find the
subarray with the largest sum, and return its sum.
 */
struct Solution;

#[allow(unused_mut)]
impl Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        println!("--------== 35 max_sub_array. nums:{:?}", nums);
        let mut prev = 0;
        let mut max = std::i32::MIN;
        let mut out: Vec<i32> = Vec::new();
        let n = nums.len();
        for i in 0..n {
            // if max > prev && prev > prev + nums[i] {
            //     println!("max = {}, prev = {}", max, prev);
            //     out.push(nums[i]);
            //     println!("adding item: {:?}", out);
            // }

            prev = nums[i].max(prev + nums[i]);
            max = max.max(prev);
            println!("max = {:?}", max);
        }
        println!("final out: {:?}", out);
        max
    }
}

#[test]
fn test53() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);

    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(Solution::max_sub_array(nums), 23);

    let nums = vec![-1];
    assert_eq!(Solution::max_sub_array(nums), -1);

    let nums = vec![1];
    assert_eq!(Solution::max_sub_array(nums), 1);
}
