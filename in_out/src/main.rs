use std::io; // to input output

fn main() {
    println!("Please input any number!");

    let mut guess = String::new();
    // let to declare variable
    // mut is for mutable variable 
    // let foo = 5; // immutable
    // let mut bar = 5; // mutable
    // The :: syntax in the ::new line indicates that new is an associated function of the String type.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read lint");
    // The & indicates that this argument is a reference

    println!("you entered: {}", guess);
}
