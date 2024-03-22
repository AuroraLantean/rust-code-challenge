/*Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 */
struct Solution;

impl Solution {
    fn is_anagram(s: String, t: String) -> bool {
        println!("--------== 242 is_anagram. s:{}, t={}", s, t);
        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();
        //s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        //t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        //  map.into_values().all(|v| v == 0)
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}

#[test]
fn test242() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}
