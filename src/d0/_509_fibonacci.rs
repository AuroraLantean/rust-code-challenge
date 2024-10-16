/*The Fibonacci sequence = Fn = Fn-1 + Fn-2:
  0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144

The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,

F(0) = 0, F(1) = 1
F(n) = F(n - 1) + F(n - 2), for n > 1.

Given n, calculate F(n).
*/
pub fn fib(n: u32) -> u32 {
  match n {
    0 => 0,
    1 => 1,
    n => fib(n - 1) + fib(n - 2),
  }
}
fn fib2(n: u32) -> u32 {
  (0..n)
    .fold((1, 0), |(nminus2, nminus1), _| (nminus1, nminus2 + nminus1))
    .1
}
fn even_sum(n: u32) -> u32 {
  let out = (1..=n).fold(0, |acc, num| if num % 2 == 0 { acc + num } else { acc });
  println!("even_sum: {out:?}");
  out
}

#[test]
fn test509() {
  println!("--------== test509");
  let n = 0;
  assert_eq!(fib(n), 0);

  let n = 1;
  assert_eq!(fib(n), 1);

  let n = 2;
  assert_eq!(fib(n), 1);

  let n = 3;
  assert_eq!(fib(n), 2);

  let n = 4;
  assert_eq!(fib(n), 3);

  let n = 15;
  println!("--------== fibonacci of {}", n);
  for int in 0..n {
    println!("{} => {}", int, fib(int));
  }
  assert_eq!(fib(n), 610);

  assert_eq!(even_sum(10), 30)
}
