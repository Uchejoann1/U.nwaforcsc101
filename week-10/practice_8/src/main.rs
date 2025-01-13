//declare a structure 
struct Employee {
    name:String,
    company:String,
    age: u32
}
fn main() {
    let emp1 = Employee{
        company:String::from("Microsoft Corporation"),
        name:String::from("satya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        name:String::from("Sundai Picai"),
        age:51
    };
    // pass emp1 and emp2 to display
     display(emp1);
     display(emp2);
 }
 fn display(emp:Employee){
    println!("Name is {}. Company is {}. age is {}",emp.name,emp.company,emp.age);
}

