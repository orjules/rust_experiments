use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    let rand_num = rand::thread_rng().gen_range(1..101);

    println!("Ich denke an eine Zahl zwischen 1 und 100!");

    loop{
        println!("Bitte gibt eine Zahl ein.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Das Programm ist abgestürzt!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Das ist keine Zahl. Versuche es nochmal.");
                continue;
            }
        };

        match guess.cmp(&rand_num){
            Ordering::Less => println!("Meine Zahl ist größer als {}. Versuche es nochmal.", guess),
            Ordering::Greater => println!("Meine zahl ist kleiner als {}. Versuche es nochmal.", guess),
            Ordering::Equal => {
                println!("Meine Zahl ist genau {}. Glückwunsch!", guess);
                break;
            }
        }
    }
}