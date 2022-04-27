mod fibonacci;
mod christmasdays;
mod kofferpacken;
mod temprechner;
mod zahlenraten;

use std::io;

fn main() {
    println!("Willkommen zu meinen ersten Experimenten mit der Rust Sprache.");

    loop {
        println!("Bitte wähle aus, was du machen möchtest:");
        println!("1. Die Fibonacci Folge bis zu einer Zahl berechnen.");
        println!("2. Den Text für '12 days of christmas' ausgeben.");
        println!("3. Das Spiel 'Ich packe meinen Koffer' spielen.");
        println!("4. Eine Temperatur von Celsius in Fahrenheit umrechnen.");
        println!("5. Das Spiel 'Zahlen raten' spielen.");
        let choice = get_number();
        match choice{
            1 => fibonacci::run(),
            2 => christmasdays::run(),
            3 => kofferpacken::run(),
            4 => temprechner::run(),
            5 => zahlenraten::run(),
            _ => println!("Etwas ist schiefgegangen!")
        }
        println!("\nIch hoffe das hat gefallen. Willst von ein anderes Programm ausprobieren Y/n?");
        if !get_yes_no(){
            println!("Auf wiedersehen.");
            break
        }
    }
}

fn get_number() -> u8{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Das Programm ist abgestürzt!");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Das ist keine Zahl. Versuche es nochmal.");
                continue;
            }
        };
        if input > 0 && input < 6 {
            return input
        }else {
            println!("Die Zahl muss zwischen 1 und 5 liegen. Versuche es nochmal.");
            continue;
        }
    }
}


fn get_yes_no() -> bool{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Das Programm ist abgestürzt!");
    input.trim() == "" || input.trim() == "y" || input.trim() == "yes"
}
