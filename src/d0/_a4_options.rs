use std::collections::HashMap;

fn lookup_eager(map: &HashMap<i32, i32>, key: i32) -> Result<i32, String> {
    map.get(&key)
        .copied()
        .ok_or(format!("key {} not found", key))
}

fn lookup_lazy(map: &HashMap<i32, i32>, key: i32) -> Result<i32, String> {
    map.get(&key)
        .copied()
        .ok_or_else(|| format!("key {} not found", key))
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    println!("--------== two_sum");
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        println!("i={}, num={}", i, num);
        if let Some(&j) = hm.get(&(target - num)) {
            return vec![j, i as i32];
        } else {
            println!("insert into map");
            hm.insert(num, i as i32);
        }
    }
    vec![]
}

#[test]
fn testa4() -> Result<(), String> {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(two_sum(nums.clone(), target), vec![0, 1]);

    let n = 10;
    let mut map = HashMap::new();
    for key in 0..n {
        map.insert(key, key);
    }

    let mut sum = 0;
    for key in 0..n {
        sum += lookup_lazy(&map, key)? as i64;
        //sum += lookup_eager(&map, key)? as i64;
    }
    println!("sum: {}", sum);
    Ok(())
}
