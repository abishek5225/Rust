use std::fmt::Display;
use std::collections::HashMap;

#[derive(Debug)]
struct User{
    name : String,
    age : u8,
    email: String,
}
#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main(){
    let mut persons = HashMap::new();
    persons.insert("Abishek", 23);
    persons.insert("Rahul", 22);
    persons.insert("Tej", 22);
    persons.insert("Yang Singh", 22);

    if let Some(person) = persons.get(person){
        println!("Abishek Scored {}", person)
    }


    
    let mut numbers = vec![1,2,3,4,5];
    numbers.push(10);
    numbers.insert(2, 20);
    println!("{:?}", numbers);
    for num in &numbers {
        println!("{:?}", num);
    }
    

   let user1= User{
    name: String::from("Abishek"),
    age: 23,
    email: String::from("abishek@gmail.com"),
   };
   println!("{:?}", user1);

   let calculate_area = Rectangle{
    width : 10,
    height : 30,
   };
   println!("{:?}", calculate_area.area());
    
}