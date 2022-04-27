use std::io;

mod numberer;
mod automaton;

fn main() {
    print_greeting();
    let input = get_input();
    if !automaton::check_input(&input){
        println!("The input was not a correctly formed roman numeral!");
        return;
    }
    let number = numberer::to_decimal(&input);
    println!("{}", number);
}

fn print_greeting(){
    println!("This rust program returns decimal equivalent of a given string of roman numerals.");
    println!("Meaning: M = 1000, D = 500, C = 100, L = 50, X = 10, V = 5, I = 1.");
    println!("The input can be uppercase, lowercase or a combination of the two.");
}

fn get_input() -> String{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading line!");
    input.truncate(input.len() - 1);
    if input.is_empty(){
        panic!("The input must not be empty.");
    }
    input.make_ascii_uppercase();
    input
}
