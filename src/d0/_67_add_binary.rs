/*Given two binary strings a and b, return their sum as a binary string.
 */
struct Solution;

impl Solution {
  fn add_binary(a: String, b: String) -> String {
    println!("--------== 67 Add Binary. a: {}, b: {},", a, b);
    let aa = u128::from_str_radix(&a, 2).unwrap_or(0);
    let bb = u128::from_str_radix(&b, 2).unwrap_or(0);
    println!("aa: {}, bb: {},", aa, bb);
    let sum = aa + bb;
    if sum - aa != bb {
      format!("{:b}", sum)+ "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    } else {
      println!("no overflow");
      println!("u128::MAX: {:?}, sum: {}", u128::MAX, sum);
      //format!("{:b}", u128::MAX);
      format!("{sum:b}")
    }
  }
}

#[test]
fn test67() {
  assert_eq!(
    Solution::add_binary("11".to_string(), "1".to_string()),
    "100".to_string()
  );
  assert_eq!(
    Solution::add_binary("1010".to_string(), "1011".to_string()),
    "10101".to_string()
  );

  assert_eq!(
        Solution::add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()),
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
}
