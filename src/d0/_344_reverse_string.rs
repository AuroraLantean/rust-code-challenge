/*Palindrome, Reverse string in vector */

fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}
fn is_palindrome(s: &str) -> bool {
    let s_lower = s.to_lowercase();
    s_lower.chars().eq(s_lower.chars().rev())
}

#[test]
fn test344() {
    let str_in = "hello";
    let mut chars_in: Vec<char> = str_in.chars().collect::<Vec<_>>();
    println!("chars_in: {:?}", chars_in);
    let str_out = "olleh";
    let chars_out: Vec<char> = str_out.chars().collect::<Vec<_>>();
    println!("chars_out: {:?}", chars_out);
    reverse_string(&mut chars_in);
    assert_eq!(chars_in, chars_out);

    let str_out2: String = chars_out.into_iter().collect(); //takes ownership //let s: String = v.iter().collect();//not taking ownership
    println!("str_out2: {}", str_out2);

    let mut input: Vec<char> = vec![];
    let output: Vec<char> = vec![];
    reverse_string(&mut input);
    assert_eq!(input, output);
    
    //-----------------==
    let str1 = "madam";
    println!("palindrome: {:?}", is_palindrome(str1));
    assert_eq!(is_palindrome(str1), true);

    let str2 = "hello".to_owned();
    assert_eq!(is_palindrome(&str2), false);
}
