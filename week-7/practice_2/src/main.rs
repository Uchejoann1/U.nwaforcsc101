use std::io;

fn main() {
    println!("Welcome!This program checks whether a character variable contains a digit or not");
    checker()
}

fn checker(){
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let ch:char = input.trim().parse().expect("invalid input");

    if ch >='0' && ch <= '9'
    {println!("Character '{}' is a digit",ch);
}
}