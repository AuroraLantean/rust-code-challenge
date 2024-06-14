/*A vending machine has the following denominations:
1c, 5c, 10c, 25c, 50c, and $1. Your task is to write a program that will be used in a vending machine to return change. Assume that the vending machine will always want to return the least number of coins or notes. Devise a function get_change(input, price) where input is how much money was inserted into the machine and price the price of the item selected, that returns an array of integers representing the number of each denomination to return.

Example:
get_change(5, 0.99) // should return [1,0,0,0,0,4]
get_change(3.14, 1.99) // should return [0,1,1,0,0,1]
get_change(3, 0.01) // should return [4,0,2,1,1,2]
get_change(4, 3.14) // should return [1,0,1,1,1,0]
get_change(0.45, 0.34) // should return [1,0,1,0,0,0]
*/
pub fn get_change(input: f64, price: f64) -> [u32; 6] {
	let input2 = (input * 100.0) as u32;
	let price2 = (price * 100.0) as u32;
	println!("{} {}", input2, price2);
	let mut t = input2 - price2;
	println!("t:{}", t);
	
	let r0 = t / 100;
	println!("r0:{}, t:{}", r0,t);
	t = t-r0*100;
	let r1 = t/ 50;
	println!("r1:{}, t:{}", r1,t);
	t = t-r1*50;
	let r2 = t / 25;
	println!("r2:{}, t:{}", r2,t);
	t = t-r2*25;
	let r3 = t / 10;
	println!("r3:{}, t:{}", r3,t);
	t = t-r3*10;
	let r4 = t/ 5;
	println!("r4:{}, t:{}", r4,t);
	t = t-r4*5;
	let r5 = t / 1;
	println!("r5:{}, t:{}", r5,t);
	let out = [r5,r4,r3,r2,r1,r0];
	println!("{:?}", out);
	out
}
#[test]
fn testquotient() {
	println!("Quotient");
	let out = get_change(5.0, 0.99);
	assert_eq!(out, [1,0,0,0,0,4]);
	let out = get_change(3.14, 1.99);
	assert_eq!(out, [0,1,1,0,0,1]);
	let out = get_change(3.0, 0.01);
	assert_eq!(out, [4,0,2,1,1,2]);
	let out = get_change(4.0, 3.14);
	assert_eq!(out, [1,0,1,1,1,0]);
	let out = get_change(0.45, 0.34);
	assert_eq!(out, [1,0,1,0,0,0]);
}