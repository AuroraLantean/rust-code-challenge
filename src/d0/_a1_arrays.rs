/* to show array syntax
 */
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
fn array_sum(arr: [i32; 5]) -> i32 {
    println!("--------== a1 array_sum arr= {:?}", arr);
    let sum = arr.iter().sum();
    sum
}
pub struct Box<'a> {
    name: &'a str,
    val: i32,
}

fn sum_struct_array() {
    let var0 = Box { name: "Aa", val: 0 };
    let var1 = Box { name: "Ab", val: 1 };
    let var2 = Box { name: "Ac", val: 2 };
    let array = [var0, var1, var2];
    let sum = array.iter().map(|var| var.val).sum::<i32>();
    println!("sum_struct_array: sum = {:?}", sum);
    //let sum = array.into_iter().map (|var| var.val).reduce(|x, y| x + y);
    //let sum: i32 = vals.into_iter().fold(0, |acc, Box {val}| acc + val);
}
//loop over struct fields
use struct_iterable::Iterable;
#[derive(Iterable)]
struct MyStruct {
    field1: u32,
    field2: String,
    field3: Option<String>,
}
fn struct_fn(instance: &MyStruct) {
    println!("----== struct_fn");
    for (field_name, field_value) in instance.iter() {
        if let Some(string_opt) = field_value.downcast_ref::<Option<String>>() {
            if let Some(str) = string_opt.as_deref() {
                println!("str: {}", str);
                println!("{}", field_name);
            }
        }
        if let Some(u32_value) = field_value.downcast_ref::<Option<u32>>() {
            println!("u32_value: {:?}", u32_value);
        }
        println!("loop end: {:?}", field_value);
    }
}

#[test]
fn testa1() {
    let arr = [11, 12, 13, 14, 15];
    assert_eq!(array_fn(arr), 0);

    let arr = [1, 2, 3, 4, 5];
    assert_eq!(array_sum(arr), 15);

    let instance = MyStruct {
        field1: 42,
        field2: "Hello, world!".to_string(),
        field3: Some("Hello, world!".to_string()),
    };
    struct_fn(&instance);

    sum_struct_array();
}
