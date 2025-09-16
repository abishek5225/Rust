use std::io;
fn main() {
    println!("Guess the number!");

    let mut guess= String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32= guess.trim().parse().expect("Please input your guess");
        println!("You guess: {guess}")

    match guess.cmp(&secret_number){
          Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        
    }
    
}
