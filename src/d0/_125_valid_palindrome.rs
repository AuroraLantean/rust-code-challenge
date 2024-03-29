/*true if after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
*/
struct Solution;

impl Solution {
    fn is_palindrome(s: String) -> bool {
        println!("--------== 125 is_palindrome. s:{}", s);
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let a: String = s.iter().collect();
        let b: String = s.iter().rev().collect();
        a == b
    }
}

#[test]
fn test125() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let res = true;
    assert_eq!(Solution::is_palindrome(s), res);

    let s = "race a car".to_string();
    let res = false;
    assert_eq!(Solution::is_palindrome(s), res);

    let s = " ".to_string();
    let res = true;
    assert_eq!(Solution::is_palindrome(s), res);
}
