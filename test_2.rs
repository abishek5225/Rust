//looking loop{}

fn main(){
let mut loop_counter = 1;
let mut while_loop_counter = 1;

while while_loop_counter <=5{
    if while_loop_counter == 3{
        while_loop_counter +=1;
        continue;
    }
    println!("{}",while_loop_counter);
    while_loop_counter +=1;
}

let test = loop{
    println!("testing loop");

    if loop_counter == 5 {
        break loop_counter;
    }

    loop_counter +=1;
};

for i in 1..=6{
    println!("for loop: {}", i);
}


}
