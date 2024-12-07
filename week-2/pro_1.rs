//Rust program to determine the annual incentive

use std::io;

fn main() {
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();
     let mut count = 0;

     
    
    for _ in 0..=50 {
        
        input1.clear();
        input2.clear();
        input3.clear();

     println!("Are you a course rep?");
     println!("true or false");

     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let course_rep:bool = input1.trim().parse().expect("Not a valid string");

    println!("Enter your cgpa");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
    let cgpa:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your level");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
    let level:f32 = input3.trim().parse().expect("Not a valid number");


    if   cgpa >4.0 && course_rep == true && level >100.0 && count <= 49 {
 
        
        println!("Enter your name");
     let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Not a valid string");
        
        println!("Enter your email");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Not a valid string");
        
        println!("Enter your department");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Not a valid string");

        println!("Enter your state_of_origin");
        let mut state_of_origin = String::new();
        io::stdin().read_line(&mut state_of_origin).expect("Not a valid string");



         println!(" You are eligble to vote");
         println!("Student details:");
        println!("Name: {}", name );
        println!("Email: {}", email );
        println!("Department: {}", department );
        println!("State of origin: {}", state_of_origin );

        name.clear();
        email.clear();
        department.clear();
        state_of_origin.clear();
        count +=1;
    }
    else {
        println!("Sorry, You are not eligble to vote");

}
println!("Number of legible candidate is {}", count);

  }  
}
