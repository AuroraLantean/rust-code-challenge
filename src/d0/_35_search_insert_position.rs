/*Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 */
struct Solution;

impl Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        println!(
            "--------== 35 search_insert index. nums:{:?}, target:{}",
            nums, target
        );
        let out = nums.binary_search(&target).unwrap_or_else(|x| x) as i32;
        println!("{:?}", out);
        out
    }
}

#[test]
fn test35() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(Solution::search_insert(nums, target), 2);

    let nums = vec![1, 3, 5, 6];
    let target = 2;
    assert_eq!(Solution::search_insert(nums, target), 1);

    let nums = vec![1, 3, 5, 6];
    let target = 7;
    assert_eq!(Solution::search_insert(nums, target), 4);

    let nums = vec![1, 3, 5, 6];
    let target = 0;
    assert_eq!(Solution::search_insert(nums, target), 0);
}
