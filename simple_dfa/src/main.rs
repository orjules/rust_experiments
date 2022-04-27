use std::io;

#[derive(Debug)]
enum States {
    Q0,
    Q1,
    Q2,
    Qtrap,
}

fn main() {
    let allowed = vec!['a','b'];

    let input = get_input();
    let mut state = States::Q0;

    for c in input.chars(){
        if ! allowed.contains(&c){
            panic!("Illegal symbol!");
        }
        state = sigma(&state, c);
    }
    match state {
        States::Q2 => println!("It is ... acceptable."),
        _ => println!("Inconceivable!"),
    }
}

fn get_input() -> String{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading line!");
    input.truncate(input.len() - 1);
    input
}

fn sigma(state:&States, c:char) -> States{
    match state {
        States::Q0 => match c {
            'a' => States::Q1,
            'b' => States::Qtrap,
            _ => panic!("Illegal symbol in sigma!"),
        }
        States::Q1 => match c {
            'a' => States::Q2,
            'b' => States::Q1,
            _ => panic!("Illegal symbol in sigma!"),
        }
        States::Q2 => match c{
            'a' | 'b' => States::Qtrap,
            _ => panic!("Illegal symbol in sigma!"),
        }
        States::Qtrap => match c {
            'a' | 'b' => States::Qtrap,
            _ => panic!("Illegal symbol in sigma!"),
        }
    }
}
