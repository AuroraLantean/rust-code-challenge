use std::collections::HashMap;

/*Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
 */
struct Solution;

impl Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        let n = nums.len() as i32;

        // Count occurrences of each number
        for num in nums {
            let count = freq_map.entry(num).or_insert(0);
            *count += 1;
        }

        // Find the majority element
        for (num, freq) in freq_map.iter() {
            if *freq > n / 2 {
                return *num;
            }
        }
        -1 // No majority element found

        /*let mut count = 0;
        let mut num: Option<i32> = None;
        for x in nums {
            if let Some(y) = num {
                if x == y {
                    count += 1;
                } else {
                    count -= 1;
                    if count == 0 {
                        num = None;
                    }
                }
            } else {
                num = Some(x);
                count += 1;
            }
        }
        num.unwrap()*/
    }
}

#[test]
fn test169() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element(nums), 2);

    let nums = vec![3, 3, 4];
    assert_eq!(Solution::majority_element(nums), 3);
}
