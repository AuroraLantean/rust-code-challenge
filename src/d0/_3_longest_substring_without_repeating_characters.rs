/*Given a string s, find the length of the longest
substring
without repeating characters.

Example 1: Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2: Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3: Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/
struct Solution;

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        println!("--------== length_of_longest_substring. s:{}", s);
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut len_longest: usize = 0;
        let mut longest: usize = 0;
        for (idx, c) in s.char_indices() {
            if let Some(prev_idx) = hm.insert(c, idx) {
                println!("prev_idx: {}, longest: {}", prev_idx, longest);
                longest = usize::max(longest, prev_idx + 1);
                println!("new longest: {}", longest);
            }
            len_longest = usize::max(idx - longest + 1, len_longest);
            println!("char: {}, idx: {}, len_longest: {}", c, idx, len_longest);
        }
        len_longest as i32
    }
}

#[test]
fn test() {
    let s = " ".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "abba".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 2);
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
    let s = "bbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
    //Solution::loop_xz()
}
/*
  fn length_of_longest_substring2(s: &str) -> u32 {
      let mut sub_str_start: usize = 0;
      let mut longest = 0;
      let mut map: HashMap<char, usize> = HashMap::new();

      for (idx, c) in s.char_indices() {
          map.entry(c)
              .and_modify(|old_idx| {
                  if *old_idx >= sub_str_start {
                      // got a repetition
                      longest = max(longest, idx - sub_str_start);
                      sub_str_start = *old_idx + 1;
                  }
                  *old_idx = idx;
              })
              .or_insert(idx);
      }
      max(longest, s.len() - sub_str_start) as u32
  }
*/
