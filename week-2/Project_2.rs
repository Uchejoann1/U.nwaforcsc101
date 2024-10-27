fn main () {
	let tq:f64 = 2.0; 
	let t = 450_000.00;
	let mq:f64 = 1.0;
	let m = 1_500_000.00;
	let hq:f64 = 3.0;
	let h = 750_000.00;
	let dq:f64 = 3.0;
	let d = 2_850_000.00;
	let aq:f64 = 1.0;
	let a = 250_000.00;

	//Summation of sales
	let ss = (tq * t) + (mq * m) + (hq * h) + (dq * d) + (aq * a);
	println!("Sum of sales ={}", ss);

	//average of sales
	let aos = ss / (tq + mq + hq + dq + aq);
	println!("average of sales ={}", aos);
}