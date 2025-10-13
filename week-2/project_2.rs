fn main () {
	let tf:f64 = 2.0; 
	let mf:f64 = 1.0;
	let hf:f64 = 3.0;
	let df:f64 = 3.0;
	let af:f64 = 1.0;

	let tx:f64 = 450000.0;
	let mx:f64 = 1500000.0;
	let hx:f64 = 750000.0;
	let dx:f64 = 2850000.0;
	let ax:f64 = 250000.0;

	// Sum of (frequency * value)
	let	sum_fx = tf * tx + mf *mx + hf * hx + df * dx + af * ax;

	// Sum of frequencies
	let sum_f = tf + mf + hf + df + af;

	// Mean (average)
	let mean = sum_fx / sum_f;

	println!("Sum of f*x = {}", sum_fx);
	println!("Sum of frequencies = {}", sum_f);
	println!("Mean = {}", mean);
}