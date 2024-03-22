/*Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array. */
struct Solution;

impl Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        println!("--------== 268 missing_number. nums:{:?}", nums);
        /*Calculate the expected sum: The expected sum of the first n natural numbers is n*(n+1)/2. This is based on the arithmetic series summation formula.
        # Iterate through the array: Traverse through the given array nums.
        # Subtract each element from the sum: For each element val in the array nums, subtract val from the sum calculated in step 1.
        # Return the remaining sum: After subtracting all elements in the array from the expected sum, what remains is the missing number.*/
        let n = nums.len() as i32;
        let mut sum = n * (n + 1) / 2;

        for val in nums {
            sum -= val;
        }
        sum

        /*     let mut xor: i32 = 0;
        let n = nums.len();
        for n in 0..=n {
            xor ^= n as i32;
        }
        for n in nums {
            xor ^= n;
        }
        xor
        */
    }
}

#[test]
fn test268() {
    let nums = vec![3, 0, 1];
    assert_eq!(Solution::missing_number(nums), 2);

    let nums = vec![0, 1];
    assert_eq!(Solution::missing_number(nums), 2);

    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(Solution::missing_number(nums), 8);
}
