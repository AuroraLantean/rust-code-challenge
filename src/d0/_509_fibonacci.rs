/*The Fibonacci sequence = Fn = Fn-1 + Fn-2:
  0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144

The Fibonacci numbers, commonly denoted F(idx) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,

F(0) = 0, F(1) = 1
F(idx) = F(idx - 1) + F(idx - 2), for idx > 1.

Given idx, calculate F(idx).
*/
fn fib(idx: i32) -> i32 {
		(0..idx)
		.fold((1, 0), |(nminus2, nminus1), _| (nminus1, nminus2 + nminus1)).1
}
fn even_sum(n: u32) -> u32 {
	let out = (1..=n).fold(0, |acc, num| if num % 2 == 0 { acc + num } else { acc });
  println!("even_sum: {out:?}");
	out
}

#[test]
fn test509() {
		println!("test509");
		let idx = 0;
    assert_eq!(fib(idx), 0);

		let idx = 1;
		assert_eq!(fib(idx), 1);

		let idx = 2;
    assert_eq!(fib(idx), 1);

		let idx = 3;
		assert_eq!(fib(idx), 2);
		
		let idx = 4;
		assert_eq!(fib(idx), 3);

		let idx = 15;
		println!("--------== fibonacci of {}", idx);
		for int in 0..idx {
			println!("{} => {}", int, fib(int));
		}
		assert_eq!(fib(idx), 610);
		
		assert_eq!(even_sum(10), 30)
}
