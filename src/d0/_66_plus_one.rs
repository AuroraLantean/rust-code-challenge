struct Solution;

impl Solution {
    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        println!("--------== 66 Plus One... digits:{:?}", digits);
        let mut out: Vec<i32> = vec![];
        let mut carry: i32 = 0;
        for (i, v) in digits.iter().rev().enumerate() {
            let x = if i == 0 { v + 1 } else { v + carry };
            carry = x / 10;
            println!("x: {}, carry: {}", x, carry);
            out.insert(0, x % 10);
        }
        if carry != 0 {
            out.insert(0, carry);
        }
        println!("out: {:?}", out);
        out
    }
}

#[test]
fn test66() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
}
