fn main() {
    let mut num:i32 = 5;
    hi(&mut num);
    println!("The value of num is : {}",num);
}

fn hi(para_num:&mut i32){
    *para_num = *para_num*0;
    println!("para_num value is :{}",para_num );
}
