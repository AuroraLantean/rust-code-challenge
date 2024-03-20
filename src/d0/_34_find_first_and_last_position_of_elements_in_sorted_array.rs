/*Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.

If target is not found in the array, return [-1, -1].
 */
struct Solution;

impl Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!(
            "--------== 34 find the first and last index. nums:{:?}, target: {}",
            nums, target
        );
        let n = nums.len();
        if let Some(idx1) = nums.iter().position(|&x| x == target) {
            println!("idx1={}", idx1);
        } else {
            println!("idx1 not found")
        };
        if let Some(idx2b) = nums.iter().rev().position(|&x| x == target) {
            println!("idx2b: {}", idx2b);
            if let Some(idx2) = (n - 1).checked_sub(idx2b) {
                println!("idx2={:?}", idx2);
            } else {
                println!("idx2 not found")
            };
        } else {
            println!("idx2b not found")
        }
        match nums.binary_search(&target) {
            //if there are multiple matches, then any one of the matches could be returned
            Ok(i) => {
                let mut l = i;
                let mut r = i;
                //sorted in non-decreasing order...
                while l > 0 && nums[l - 1] == target {
                    l -= 1;
                }
                while r + 1 < n && nums[r + 1] == target {
                    r += 1;
                }
                vec![l as i32, r as i32]
            }
            Err(_) => vec![-1, -1],
        }
    }
}

#[test]
fn test34() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    let res = vec![3, 4];
    assert_eq!(Solution::search_range(nums, target), res);
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;
    let res = vec![-1, -1];
    assert_eq!(Solution::search_range(nums, target), res);
    let nums = vec![];
    let target = 0;
    let res = vec![-1, -1];
    assert_eq!(Solution::search_range(nums, target), res);
}
