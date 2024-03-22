/*Given an integer n, return a string array answer (1-indexed) where:

    answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
    answer[i] == "Fizz" if i is divisible by 3.
    answer[i] == "Buzz" if i is divisible by 5.
    answer[i] == i (as a string) if none of the above conditions are true.
*/
struct Solution;

impl Solution {
    fn fizz_buzz(n: i32) -> Vec<String> {
        println!("--------== 412 fizz_buzz n= {:?}", n);
        let mut res = vec![];
        for i in 1..=n {
            let fizz = i % 3 == 0;
            let buzz = i % 5 == 0;
            let s = match (fizz, buzz) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                _ => i.to_string(),
            };
            res.push(s);
        }
        res
    }
}

#[test]
fn test412() {
    let output: Vec<&str> = vec!["1", "2", "Fizz"];
    assert_eq!(Solution::fizz_buzz(3), output);

    let output: Vec<&str> = vec!["1", "2", "Fizz", "4", "Buzz"];
    assert_eq!(Solution::fizz_buzz(5), output);

    let output: Vec<&str> = vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
        "FizzBuzz",
    ];
    assert_eq!(Solution::fizz_buzz(15), output);
}
