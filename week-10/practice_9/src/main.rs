//define dimensions of a rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32, height:u32
}
//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self)->u32 {
       // use the . operator to fecth the value of a fieldvia the self keyword
        self.width * self.height
    }
}
fn main(){
    //instanatiate the structure
    let small = Rectangle {
        width:10,
        height:20
    };
    //print the rectangle area
    println!("width is {} \n height is {} \n area of rectangle 
        is {}",small.width,small.height,small.area());
}
