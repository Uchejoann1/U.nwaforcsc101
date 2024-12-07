//Rust program to determine the annual incentive

use std::io;

fn main() {
     let mut input2 = String::new();
     let mut input1 = String::new();
      let mut count = 0;
while count<3 {
     println!("Name");
     io::stdin().read_line(&mut input1).expect("Not a valid number");

    println!("Enter your no. of papers");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
    let papers:f32 = input2.trim().parse().expect("Not a valid number");

    if papers >=10.0 {
        println!("The incentive is N1,000,000.00");
    }
    else if papers >5.0 && papers <10.0{
        println!("The incentive is N800,000.00");
    }
    else if papers >=3.0 && papers <= 5.0
{
        println!("The incentive is N500,000.00");
    }
    else {
        println!("The incentive is N100,000.00"); }
          count+=1;
}

}
      