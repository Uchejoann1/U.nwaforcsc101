fn main() {
    // using Vec::new()
    let v:Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\nThe length of vec::new is: {}",v.len());

    //using macro
    let v = vec!["grace", "uc", "uche", "susan", "basil"];

    //printing the size of vector
    println!("\nThe length of vec macro is: {}",v.len() );
}
