/*Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
 */
use core::num;

struct Solution;

impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        println!("--------== remove_duplicates. nums:{:?}", nums);
        if nums.is_empty() {
            return 0;
        }

        let mut j = 0;
        for i in 1..nums.len() {
            //nums is sorted in non-decreasing order
            if nums[j] != nums[i] {
                j += 1;
                nums[j] = nums[i];
            }
        }

        (j + 1) as i32
        //     let n = nums.len();
        //     if n == 0 {
        //         return 0;
        //     }
        //     let mut last = nums[0];
        //     let mut size = 1;
        //     for i in 1..n {
        //         if nums[i] != last {
        //             last = nums[i];
        //             nums[size] = nums[i];
        //             size += 1;
        //         }
        //     }
        //     nums.resize(size, 0);
        //     nums.shrink_to_fit();
        //     size as i32
    }
}

#[test]
fn test26() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
}
