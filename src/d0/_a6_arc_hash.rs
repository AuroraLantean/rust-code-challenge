use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Cache {
  data: HashMap<String, i32>,
}

impl Cache {
  fn new() -> Self {
    Cache {
      data: HashMap::new(),
    }
  }
  fn insert(&mut self, key: String, value: i32) {
    self.data.insert(key, value);
  }
  fn get(&self, key: &str) -> Option<i32> {
    self.data.get(key).copied()
  }
}

fn fn_one() {
  let cache = Arc::new(Mutex::new(Cache::new()));
  let mut handles = vec![];

  // Writer threads
  for i in 0..3 {
    let cache = Arc::clone(&cache);
    handles.push(thread::spawn(move || {
      let mut cache = cache.lock().unwrap();
      cache.insert(format!("key{}", i), i);
      println!("Thread {} wrote key{}", i, i);
    }));
  }

  // Reader thread
  let cache_reader = Arc::clone(&cache);
  handles.push(thread::spawn(move || {
    thread::sleep(std::time::Duration::from_millis(100)); // Wait for writes
    let cache = cache_reader.lock().unwrap();
    println!("Read: key1 = {:?}", cache.get("key1"));
  }));

  for handle in handles {
    handle.join().unwrap();
  }

  let cache = cache.lock().unwrap();
  println!("Final cache: {:?}", cache.data);
}
/*
Thread 0 wrote key0
Thread 1 wrote key1
Thread 2 wrote key2
Read: key1 = Some(1)
Final cache: {"key0": 0, "key1": 1, "key2": 2}
*/
