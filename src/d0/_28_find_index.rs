/* Find the index of the first occurrence in a String, or -1 if there is no such occurrence
 */
struct Solution;

impl Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        println!(
            "--------== 28_Find Occurrence Index. haystack:{}, needle:{}",
            haystack, needle
        );
        match haystack.find(&needle) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

#[test]
fn test28() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(
        Solution::str_str("aaaaa".to_string(), "bba".to_string()),
        -1
    );
    assert_eq!(
        Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
        0
    );

    assert_eq!(
        Solution::str_str("leetcode".to_string(), "leeto".to_string()),
        -1
    );
}
