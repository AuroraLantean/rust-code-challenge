/* to show vector syntax
 */
struct Solution;

fn vec_fn(mut vetr: Vec<i32>) -> i32 {
    println!("--------== a1 vec_fn vetr= {:?}", vetr);

    for v in vetr.clone() {
        println!("v={}", v);
    }

    //use this to update Rust 2015 and 2018 to Rust 2021.
    for v in vetr.iter() {
        println!("iter v={}", v);
    }

    //flexible: if vetr is by reference(mut), x will be by reference(mut)!
    for (idx, v) in (&mut vetr).into_iter().enumerate() {
        println!("idx={}, v={}", idx, v);
    }
    println!("vetr: {:?}", vetr);
    0
}

#[test]
fn testa2() {
    let mut vetr = Vec::new();
    vetr.push(0);
    vetr.push(1);
    vetr.push(2);
    assert_eq!(vec_fn(vetr), 0);
}
