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

    //reverse a vector
    let m1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let last_one = m1.last().copied().unwrap();
    println!("last element:{:?}", last_one);
    assert_eq!(last_one, 5);
    
    let m2: Vec<i32> = m1.into_iter().rev().collect();
    println!("reversed vector: {:?}", m2);
    assert_eq!(vec![5, 4, 3, 2, 1], m2);
    
    //sort a vector
    let mut v = [5, 3, 1, 4, 2];
    v.sort();
    println!("sorted vector: {:?}", v);
    assert_eq!(v, [1, 2, 3, 4, 5]);

    //find the index of an element in an array, vector or slice
    let mut v3 = vec!["one", "two", "three"];
    let index = v3.iter().position(|&r| r == "two").unwrap();
    println!("element index: {}", index);
    assert_eq!(index, 1);
    
    //remove an element via its index, assuming the element exists
    v3.remove(index);
    println!("v3 after removing: {:?}", v3);
    assert_eq!(v3, ["one", "three"]);
    
    //remove an element via its value, assuming the element exists
    let mut xs = vec![2, 1, 2, 3, 2];
    let some_x = 2;
    xs.retain(|&x| x != some_x);
    println!("xs after removing: {:?}", xs);
    assert_eq!(xs, [1, 3]);
    
    let var0 = Box { name: "Aa", val: 0 };
    let var1 = Box { name: "Ab", val: 1 };
    let var2 = Box { name: "Ac", val: 2 };
    let vetr = vec![var0, var1, var2];
    assert_eq!(sum_struct_vec(vetr), 3);
    
}
