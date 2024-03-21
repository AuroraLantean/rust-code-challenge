/*An n-bit gray code sequence is a sequence of 2n integers where:

    Every integer is in the inclusive range [0, 2n - 1],
    The first integer is 0,
    An integer appears no more than once in the sequence,
    The binary representation of every pair of adjacent integers differs by exactly one bit, and
    The binary representation of the first and last integers differs by exactly one bit.

Given an integer n, return any valid n-bit gray code sequence.
 */
struct Solution;

impl Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        println!("--------== 89 gray_code. n:{:?}", n);
        let mut res = vec![];
        for i in 0..(1 << n) {
            res.push(i ^ i >> 1);
        }
        res
    }
}

#[test]
fn test89() {
    let n = 2;
    let res = vec![0, 1, 3, 2];
    assert_eq!(Solution::gray_code(n), res);

    let n = 1;
    let res = vec![0, 1];
    assert_eq!(Solution::gray_code(n), res);
}
