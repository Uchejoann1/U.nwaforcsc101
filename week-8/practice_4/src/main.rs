fn main() {
    // Name vector
    let name = vec!["Mary","sam","john","sally","uche","blessing","chiboy","paul"];
    //Age vector
    let age = vec![16,17,18,19,20,21,22,23];

    print!("\nAge allocation:\n");
    for i in 0..age.len()
    {
        //iterating through i on the vector
        print!("{} is {} years old\n",name[i],age[i] );
    }
}