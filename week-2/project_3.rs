<<<<<<< HEAD
fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// compound interest
	let a = p * (1.0 - (r / 100.0)).powf(t);
	println!("Amount {}", a);
	let ci = a - p;
	println!("Compound Interest {}", ci);

=======
fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// compound interest
	let a = p * (1.0 - (r / 100.0)).powf(t);
	println!("Amount {}", a);
	let ci = a - p;
	println!("Compound Interest {}", ci);

>>>>>>> b3ecda49d6429086cbd227ee208a8812864a1671
}