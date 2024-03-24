use core::num;
use std::collections::HashMap;

/*Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
 */
fn majority_element(nums: &Vec<i32>) -> i32 {
    println!("--------== majority_element");
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    let n = nums.len() as i32;

    // Count occurrences of each number
    for num in nums {
        let count = freq_map.entry(*num).or_insert(0);
        *count += 1;
        //*freq_map.entry(*x).or_default() += 1;
    }

    // Find the majority element
    for (num, freq) in freq_map.iter() {
        if *freq > n / 2 {
            return *num;
        }
    }
    -1 // No majority element found
}
fn most_frequent(nums: &Vec<i32>) -> (i32, usize) {
    println!("--------== most_frequent");
    let mut freq_map: HashMap<i32, usize> = HashMap::new();
    for x in nums {
        *freq_map.entry(*x).or_default() += 1;
    }

    //iterate over the map to find the key with the highest value:
    let max = freq_map
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, v)| (k, v));
    max.unwrap_or_else(|| (0, 0))
}

#[test]
fn test169() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(majority_element(&nums), 2);
    assert_eq!(most_frequent(&nums), (2, 4));

    let nums = vec![3, 3, 4];
    assert_eq!(majority_element(&nums), 3);
    assert_eq!(most_frequent(&nums), (3, 2));
}
