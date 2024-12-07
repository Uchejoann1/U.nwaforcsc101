fn main() {
    let num:i32 = 5;
    hi(num);
    println!("The value of no is: {}",num);
}
fn hi(mut para_num: i32){
    para_num = para_num*0;
    println!("para_num value is :{}", para_num);
}
