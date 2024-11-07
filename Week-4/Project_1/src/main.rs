// Rust program to find  the roots of a quadratic equation 

use std::io;

fn main() {
     let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter co-eff of x^2 ");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter co-eff of x ");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter the constant ");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = b.powi(2) - 4.0*a*c;
    println!("Discriminant of the equation is {}", d);
     let root1:f32 = ((-b) + d.powi(1/2)) / (2.0 * a);
      let root2:f32 = ((-b) - d.powi(1/2)) / (2.0 * a);
    println!("The roots of the equation are {} , {}", root1 , root2);

    if d > 0.0
    { 
        println!("The roots of the equation are two distinct roots");
    } 
    else if d == 0.0
    {
        println!("The root of the equation is exactly one real root");
    }
    else 
    {
        println!("There are no real roots");
    }
}
