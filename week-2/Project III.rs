fn main () {
	let p:f64 = 510_000.0
	let r:f64 = 5.0
	let n:f64 = 3.0

	// Depreciation
	let A = p*(1-(r/100))^n;
	println!("the depreciation is {}", A);
	let V = p - A;
	println!("The value of the TV after tree years is {}", V);
}