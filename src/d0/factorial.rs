/*You have been given a positive integer N. You need to find and print the Factorial of this number. The Factorial of a positive integer N refers to the product of all number in the range from 1 to N. You can read more about the factorial of a number here.

Input Format:
The first and only line of the input contains a single integer N denoting the number whose factorial you need to find.

Output Format
Output a single line denoting the factorial of the number N.

Constraints: 1 <= N <= 10
*/
pub fn factorial(num: u128) -> u128 {
	(1..=num).product()
}

#[test]
fn factorial_of_x() {
  assert_eq!(1,factorial(1));
  assert_eq!(2,factorial(2));
  assert_eq!(51090942171709440000,factorial(21));
}

