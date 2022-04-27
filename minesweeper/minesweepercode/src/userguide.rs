use std::io;

pub fn say_hello(bombcount: u16){
    println!("Willkommen bei Minesweeper.");
    println!("Durch die Eingabe von Koordinaten wie '1, 2' oder '3,4' oder sogar '02' können\
     Felder aufgedeckt werden. Dabei ist die erste Zahl die Zeile und die zweite Spalte");
    println!("Auf der Karte sind {} Bomben versteckt.", bombcount);
    println!("Zum Markieren eines Feldes als Bombenfeld muss vor die Koordinaten ein \
    @-Symbol gestellt werden also z.B. '@ 1,2'.");
    println!("Das Spiel ist gewonnen, wenn alle Bomben markiert sind.");
    println!("Viel Spaß!");
}

pub fn get_input(size: u16) -> (bool, u16, u16){

    // ACHTUNG!! Das hier funktioniert nur mit einem 9x9 Feld, alle darüber braucht einen neuen Parser

    // Erlaubt sind: zwei Zahlen ohne Zusatz oder zwei Zahlen mit einem @ davor bzw danach
    // Variablen festlegen
    let mut marker_flag = false;
    let mut input_position : Vec<u16>= Vec::new();
    // Loop starten um nach Falscheingabe wiederholen zu können
    loop {
        // Input holen
        input_position.clear();
        let mut usr_input = String::new();
        io::stdin().read_line(&mut usr_input).expect("Ungültige Eingabe");
        let input_list = usr_input.chars();
        // Input prüfen
        for input in input_list {
            if input.eq_ignore_ascii_case(&'@'){
                marker_flag = true;
            }
            let number: u16 = match input.is_digit(10) {
                true => input.to_digit(10).unwrap() as u16,
                false => continue
            };
            if number >= 0 && number <= size{
                input_position.push(number);
            }
        }
        if input_position.len() == 2{
            break
        }else {
            println!("Inkorrekte Eingabe, bitte noch einmal!")
        }
    }
    // Input zurückgeben
    (marker_flag, input_position[0], input_position[1])
}