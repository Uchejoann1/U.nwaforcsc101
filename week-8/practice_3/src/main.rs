fn main() {
    let v = vec!['c','d','f','w','e','j','r','t','y'];
    let mut input1 = String::new();

    println!("Enter an index value btw (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    //index is the non negative value which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid input");
    
    // getting value at siven index value
    let ch:Option<&char> = v.get(index);
    value(ch);
}
fn value(n:Option<&char>)
{println!("Element of vector {:?}",n );
}