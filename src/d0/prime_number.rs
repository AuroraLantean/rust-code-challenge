/* Check if the given string is a prime number
*/
pub fn is_prime(num: String) -> bool {
	println!("is_prime input: {}", num);
  if let Ok(num) = num.parse() {
    return is_prime_u8(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u16(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u32(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u64(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u128(num);
  }

  false
}

fn is_prime_u8(num: u8) -> bool {
  if num == 0 || num == 1 {
    return false;
  }
	let mut i = 2;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 1;
  }
  true
}

fn is_prime_u16(num: u16) -> bool {
  if num == 0 || num == 1 {
    return false;
  }
	let limit = (num as f32).sqrt() as u16;
	for i in 2..=limit {
		if num % i == 0 {
				return false;
		}
	}
	true
}

fn is_prime_u32(num: u32) -> bool {
  if num == 0 || num == 1 {
    return false;
  }
	let limit = (num as f64).sqrt() as u32;
	for i in 2..=limit {
		if num % i == 0 {
				return false;
		}
	}
  true
}

fn is_prime_u64(num: u64) -> bool {
  if num == 0 || num == 1 {
    return false;
  }
	//u128 does not have sqrt()
  let mut i = 2;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }
    i += 1;
  }

  true
}

fn is_prime_u128(num: u128) -> bool {
  if num == 0 || num == 1 {
    return false;
  }
	//u256 does not exist
  let mut i = 2;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }
    i += 1;
  }

  true
}

#[test]
fn test_prime() {
	let input = "0".to_string();
  assert_eq!(is_prime(input), false);
	let input = "1".to_string();
  assert_eq!(is_prime(input), false);
	let input = "2".to_string();
  assert_eq!(is_prime(input), true);
	let input = "3".to_string();
  assert_eq!(is_prime(input), true);
	let input = "4".to_string();
  assert_eq!(is_prime(input), false);
	let input = "5".to_string();
  assert_eq!(is_prime(input), true);
	let input = "6".to_string();
  assert_eq!(is_prime(input), false);
	let input = "7".to_string();
  assert_eq!(is_prime(input), true);
	let input = "8".to_string();
  assert_eq!(is_prime(input), false);
	let input = "293".to_string();
  assert_eq!(is_prime(input), true);
}

