use std::io;

//Define a struct for Staff
 
 struct Staff {
 name: String,
papers_published: u32,}
// implement method to calculate incentive

impl Staff {
	fn calculate_incentive(&self)
-> u32 {
	match
	self.papers_published {
	0..=2 => 100_000,
	3..=5 => 500_000,
	6..=9 => 800_000,
	_=> 1_000_000,
	}
}
}
fn get_user_input(prompt: &str) ->
String {
	println!("{}", prompt);
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("failed to read");
	input.trim().to_string()
}
fn get_numeric_input(prompt: &str) ->u32 { 
	loop {
	let input = get_user_input(prompt);
	match input.parse::<u32>(){ Ok(num) => return num, 
		Err(_) => println!("Invalid input. Please enter a number."),
}
}}
// Main function
fn main(){
	let name =get_user_input("Enter your name:");
	let papers_published = get_numeric_input("Enter number of papers published:");
	let staff =Staff {name,  papers_published,};

	//input from main program
	let mut _count = 0;
	for  i in 1..=100 {
	 let _name_1 = format!("staff {}", i);
	 _count +=1
	}
	 let incentive = staff.calculate_incentive();
	println!("{} has published {} papers and received an incentive of N{}", staff.name, staff.papers_published, incentive);}