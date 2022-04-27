use std::io;
use std::vec::Vec;

pub fn run() {

    // Idee ist ein Spiel im Sinne von "Ich packe meinen Koffer..."
    // Dann muss der Nutzer eine Sache eingeben
    // Dann wird weitergegeben, bzw einfach weiter gemacht
    // Bevor etwas neues eingegeben werden kann müssen alle bisherigen Dinge nacheinander
    // eingegeben werden. Dabei wird abgebrochen und der Score angezeigt, wenn etwas falsch war
    // Wichtig: Die Console muss nach jedem Durchlauf gecleared werden

    let mut dinge = Vec::new();
    let mut durchlauf = 0;

    println!("Ich packe meinen Koffer und nehme mit...");
    let mut usr_input = String::new();
    io::stdin().read_line(&mut usr_input).expect("Ungültige Eingabe");
    dinge.push(usr_input);
    print!("\x1B[2J\x1B[1;1H");
    println!("Ok. Jetzt musst du das bisherige aufzählen, bevor du was neues hinzufügen kannst.");

    'outer: loop{
        println!("Ich packe meinen Koffer und nehme mit...");
        for index in 0..dinge.len() {
            let mut usr_input = String::new();
            io::stdin().read_line(&mut usr_input).expect("Ungültige Eingabe");
            match usr_input == dinge[index]{
                true => continue,
                _ => println!("Leider falsch. Das war dein {}ter Durchlauf.", durchlauf)
            }
            break 'outer;
        }
        println!("Korrekt. Und du fügst hinzu...");
        let mut usr_input = String::new();
        io::stdin().read_line(&mut usr_input).expect("Ungültige Eingabe");
        dinge.push(usr_input);
        durchlauf += 1;
        print!("\x1B[2J\x1B[1;1H");
        println!("Nächste Runde!");
    }
}
