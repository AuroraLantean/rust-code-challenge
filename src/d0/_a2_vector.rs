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
fn vector_sum(vetr: Vec<i32>) -> i32 {
    println!("--------== a2 vector_sum vetr= {:?}", vetr);
    let sum = vetr.iter().sum();
    sum
}
pub struct Box<'a> {
    name: &'a str,
    val: i32,
}
fn sum_struct_vec(vetr: Vec<Box>) -> i32 {
    let sum = vetr.iter().map(|var| var.val).sum::<i32>();
    println!("sum_struct_vec: sum = {:?}", sum);
    //let sum = array.into_iter().map (|var| var.val).reduce(|x, y| x + y);
    //let sum: i32 = vals.into_iter().fold(0, |acc, Var {val}| acc + val);
    sum
}
#[test]
fn testa2() {
    let mut vetr = Vec::new();
    vetr.push(0);
    vetr.push(1);
    vetr.push(2);
    assert_eq!(vec_fn(vetr), 0);

    let vetr: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert_eq!(vector_sum(vetr), 15);

    let var0 = Box { name: "Aa", val: 0 };
    let var1 = Box { name: "Ab", val: 1 };
    let var2 = Box { name: "Ac", val: 2 };
    let vetr = vec![var0, var1, var2];
    assert_eq!(sum_struct_vec(vetr), 3);
}
