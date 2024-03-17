struct Solution;

use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!("--------== two_sum");
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            println!("i={}, num={}", i, num);
            if let Some(&j) = hm.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                println!("insert into map");
                hm.insert(num, i as i32);
            }
        }
        vec![]
    }

    fn two_sum_with_for_loop(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!("--------== two_sum_with_for_loop");
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![] //unreachable!()
    }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums.clone(), target), vec![0, 1]);
    assert_eq!(Solution::two_sum_with_for_loop(nums, target), vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums.clone(), target), vec![1, 2]);
    assert_eq!(Solution::two_sum_with_for_loop(nums, target), vec![1, 2]);
}
