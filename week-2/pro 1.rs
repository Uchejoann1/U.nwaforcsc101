//Rust program to determine the annual incentive

use std::io;

fn main() {
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();

     println!("Are you a course rep?");
     println!("Yes = 1 ;  No = 0");

     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let course_rep:bool = input1.trim().parse().expect("Not a valid string");
    println!("Enter your age");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
    let cgpa:u32 = input2.trim().parse().expect("Not a valid number");
    println!("Enter your level");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
    let level:u32 = input3.trim().parse().expect("Not a valid number");


    if cgpa >=4.0 && work == 1.0 && level >100.0 {


        println!(" You are eligble to vote");
    }
    else {
        println!("Sorry, You are not eligble to vote");
    }
}
