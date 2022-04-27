use std::io;

pub fn run() {
    // Dieses Programm soll eine Eingabe in Fahrenheit nehmen und in Celsius umrechnen
    // Zur Grundlage dient diese Formel: °C = (°F - 32) * 5/9

    println!("Bitte gib eine Zahl in Fahrenheit ein und ich sage die das Äquivalent in Celsius.");
    let num_fahrenheit = get_input();
    let num_celsius = (num_fahrenheit - 32.0) * (5.0/9.0);
    println!("Das sind dann {} Grad Celsius.", num_celsius);
}

fn get_input() -> f32{
    loop{
        let mut num_fahrenheit = String::new();
        io::stdin()
            .read_line(&mut num_fahrenheit)
            .expect("Failed to read line");
        match num_fahrenheit.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Ungültige Eingabe. Bitte nochmal."),
        };
    }
}