//Rust program to determine the annual incentive

use std::io;

fn main() {
     let mut input1 = String::new();
     let mut input2 = String::new();

     println!("Are you experienced or inexperienced?");
     println!("Experienced = 1 ;  Inexperienced = 0");

     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let work:f32 = input1.trim().parse().expect("Not a valid string");
    println!("Enter your age");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if age >=40.0 && work == 1.0 {
        println!("The incentive is N1,560,000.00");
    }
    else if age >=30.0 && age <40.0 && work ==1.0{
        println!("The incentive is N1,480,000.00");
    }
    else if age <30.0 && work == 1.0
{
        println!("The incentive is N1,300,000.00");
    }
    else {
        println!("The incentive is N100,000.00"); }
}
