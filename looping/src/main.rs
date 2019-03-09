use std::io; // to input output
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 10);
    
    loop {
        println!("Please input your guess(1-10)!");

        let mut guess = String::new();
        // let to declare variable
        // mut is for mutable variable 
        // let foo = 5; // immutable
        // let mut bar = 5; // mutable
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read lint");
        // The & indicates that this argument is a reference

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // again used let to convert value of same variable from one type to another type.(sting to number or vice versa)

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
