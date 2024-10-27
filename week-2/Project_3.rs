fn main () {
	let p:f64 = 510_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// Depreciation
	let a = p * (1.00 - (r / 100.0)) .powf(n);
	println!("the depreciation is {}", a);
	let v = p - a;
	println!("The value of the TV after tree years is {}", v);
}