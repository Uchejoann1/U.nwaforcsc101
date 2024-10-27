fn main () {
	let Tq:f64 = 2.0; 
	let T = 450_000.00;
	let Mq:f64 = 1.0;
	let M = 1_500_000.00;
	let Hq:f64 = 3.0;
	let H = 750_000.00;
	let Dq:f64 = 3.0;
	let D = 2_850_000.00;
	let Aq:f64 = 1.0;
	let A = 250_000.00;

	//Summation of sales
	let ss = (Tq * T) + (Mq * M) + (Hq * H) + (Dq * D) + (Aq * A);
	println!("Sum of sales ={}", ss);

	//Average of sales
	let as = ss / (Tq + Mq + Hq + Dq + Aq);
	println!("Average of sales ={}", as);
}