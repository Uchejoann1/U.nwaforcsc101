use std::io;
fn main() {
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");
    add(a, b);
}

fn add(z: i32, x: i32){
    let sum = z + x;
    println!("Sum of A and B = {}", sum );
}
