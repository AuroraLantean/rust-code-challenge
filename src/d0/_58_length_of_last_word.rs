/*Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal substring consisting of non-space characters only.
*/
struct Solution;

impl Solution {
    fn length_of_last_word(s: String) -> i32 {
        println!("length of last word. s = {}", s);
        if let Some(last) = s.split_whitespace().last() {
            last.len() as i32
        } else {
            0
        }
    }
}

#[test]
fn test58() {
    assert_eq!(
        Solution::length_of_last_word(String::from("Hello World")),
        5
    );
    assert_eq!(
        Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
        4
    );
    assert_eq!(
        Solution::length_of_last_word(String::from("luffy is still joyboy")),
        6
    );
}
