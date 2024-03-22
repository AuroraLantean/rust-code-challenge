/*Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.*/
struct Solution;

use std::collections::HashSet;

impl Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        println!("--------== 217 contains_duplicate nums= {:?}", nums);
        let mut hs: HashSet<i32> = HashSet::new();
        for n in nums {
            if hs.contains(&n) {
                return true;
            } else {
                hs.insert(n);
            }
        }
        false
    }
}

#[test]
fn test217() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::contains_duplicate(nums), true);

    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::contains_duplicate(nums), false);

    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert_eq!(Solution::contains_duplicate(nums), true);
}
