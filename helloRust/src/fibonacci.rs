use std::io;

pub fn run() {
    // Die Fibonacci Folge funktioniert mit der Addition der Vorherigen beiden Zahlen
    // Also 0, 1 sind fest, dann 0+1=1, dann 1+1=2, dann 1+2=3, dann 2+3=5, dann 3+5=8, usw
    // Das ganze sollte man wahrscheinlich rekursiv implementieren

    println!("Bitte gibt eine ganze Zahl ein. Diese nutze ich als Stelle für die zugehörige Fibonacci Zahl.");
    let input = get_input();
    as_loop(input);
}

fn get_input() -> u32{
    loop{
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        match n.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Ungültige Eingabe. Bitte nochmal."),
        };
    }
}

fn as_loop(n: u32){
    let mut out_vec = vec![0, 1];
    for index in 1..n-1{
        let new_num = out_vec[index as usize] + out_vec[(index-1) as usize];
        out_vec.push(new_num);
    }
    println!("Die Abfolge bis zur {}ten Zahl ist: ", n);
    println!("{:?}", out_vec)
}