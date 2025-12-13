//ownership & Borrowing
fn main(){
    let mut name = String::from("Abishek Chaulagain");//name owns the string "abishek"
    changeName(&mut name);
    println!("{}", name);
    /*
let name = String::from("Abishek");//name owns the string "abishek"
let name1 = name;
let name2 = name1.clone();// it just clones the value
// println!("{}", name);
println!("{}", name1);
println!("{}", name2);
*/

//printName(&name);//borrowed to the function but still owner remains "name" because of &
//println!("Checking: {}", name);

}
/*
fn printName(name: &String){
    println!("{}", name);
}
    */

fn changeName(name: &mut String){
    name.push_str("David Eagleman")
}