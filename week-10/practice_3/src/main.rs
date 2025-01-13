fn main() {
let v = vec![20,40, 60, 80];
    //vector v owns the object in heap

    let v2 =v.clone();  //moves ownership to v2

    let _v2_return = display(v2);
    //v2 is moved to display and v2 is invalidated

    println!("in main {:?}",v);
    //v2 is no longer usable here
}
fn display(v:Vec<i32>)->Vec<i32> {
    //returning same vector
    println!("inside display {:?}",v);
    return v;
}
