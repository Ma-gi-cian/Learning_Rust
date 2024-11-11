use std::io; // standard library documentation - prelude

fn main() {
    println!("Guess a Number !");
    println!("Enter the Number :-");

    let mut number = String::new(); // let is the declaration of variables 
    // variables are by default of immutable type in rust and hence use mut to declare a mutable variable
    // = is = ( equal to is equal to )
    // String::new() -> string is a class and new is a function that assigns an empty string
    io::stdin().read_line(&mut number).expect("You are inside expect ?? this is supposed to be error handling not the normal flow of the program");

    println!("Number : {}", number);
}
