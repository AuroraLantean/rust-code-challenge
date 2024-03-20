use core::num;

/*Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

    Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
    Return k.
*/
struct Solution;

impl Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        println!(
            "--------== 27 remove_element. nums:{:?}, val: {}",
            nums, val
        );
        let mut size: usize = 0;
        //for (i, v) in nums.into_iter().enumerate() {}
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[size] = nums[i];
                if i != size {
                    nums[i] = -1; //remove all occurrences of val in nums in-place(in itself)
                }
                println!("size: {}", size);
                size += 1;
            } else {
                println!("found match on index {}", i);
                nums[i] = -1;
            }
            println!("nums:     {:?}", nums);
        }
        size as i32
    }
}

#[test]
fn test27() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    assert_eq!(Solution::remove_element(&mut nums, val), 2);
    println!("expected: [2, 2, _, _]");

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    assert_eq!(Solution::remove_element(&mut nums, val), 5);
    println!("expected: [0, 1, 3, 0, 4, _, _, _]");
}
