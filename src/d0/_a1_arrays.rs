/* to show array syntax
 */
struct Solution;

fn array_fn(mut arr: [i32; 5]) -> i32 {
    println!("--------== a1 array_fn arr= {:?}", arr);

    for v in arr {
        println!("v={}", v);
    }

    //use this to update Rust 2015 and 2018 to Rust 2021.
    for v in arr.iter() {
        println!("iter v={}", v);
    }

    //flexible: if arr is by reference(mut), x will be by reference(mut)!
    for v in (&mut arr).into_iter() {
        println!("into_iter v={}", v);
    }
    0
}

#[test]
fn testa1() {
    let arr = [11, 12, 13, 14, 15];
    assert_eq!(array_fn(arr), 0);
}
