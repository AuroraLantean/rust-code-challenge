/*Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red(0), white(1), and blue(2).

We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.

You must solve this problem without using the library's sort function.
 */
struct Solution;

impl Solution {
    fn sort_colors(nums: &mut Vec<i32>) {
        println!("--------== 75 Sort Colors. nums:{:?}", nums);
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut i = 0;
        while i <= r {
            while nums[i] == 2 && i < r {
                nums.swap(i, r); // sort them in-place. Move this "2" to the last index as r == n-1
                r -= 1; //reduce the last index
            }
            while nums[i] == 0 && i > l {
                nums.swap(i, l); // sort them in-place
                l += 1;
            }
            i += 1;
        }
    }
}

#[test]
fn test75() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    let res = vec![0, 0, 1, 1, 2, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, res);

    let mut nums = vec![2, 0, 1];
    let res = vec![0, 1, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, res);
}
