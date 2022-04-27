use crate::fieldgenerator::change_bomb_on_field;
use crate::revealer::{add_mark_to_map, change_map, is_marked, is_revealed, remove_mark_on_map};

mod fieldgenerator;
mod fieldprinter;
mod revealer;
mod userguide;

const SIZE: u16 = 9;
const BOMB_COUNT: u16 = 9;

fn main() {
    let field = fieldgenerator::get_field(SIZE, BOMB_COUNT);
    fieldprinter::print_real(&field);
    let map = revealer::make_revealed_map(SIZE);
    run_game(field, map, SIZE);
}

fn run_game(field: Vec<Vec<u16>>, map: Vec<Vec<char>>, size: u16){
    let mut map= map;
    let mut field = field;
    userguide::say_hello(BOMB_COUNT);
    loop {
        fieldprinter::print_map(&map);
        let input_tuple = userguide::get_input(size);
        let coord = (input_tuple.1, input_tuple.2);
        let coord_sizes = (input_tuple.1 as usize, input_tuple.2 as usize);
        if input_tuple.0{
            // Markierung wurde gesetzt oder entfernt
            // Als erstes prüfen ob schon angewählt
            if is_marked(coord_sizes, map) {
                map = remove_mark_on_map(coord, map);
                field = change_bomb_on_field(coord_sizes, field);
            }else {
                // Sonderfall des Markierens von schon aufgedeckten Feldern abfangen
                if is_revealed(coord_sizes, map){
                    // TODO schöner machen, weil das jetzt vor der neuen Map geprinted wird
                    // Idee: Beim userguide einen Satz hinterlassen, die vor der nächsten Input abfrage gedruckt wird
                    println!("Markieren ist nur auf nicht aufgedeckten Feldern erlaubt!");
                    continue;
                }
                map = add_mark_to_map(coord, map);
                field = change_bomb_on_field(coord_sizes, field);
            }
            // if is_done { }
        }else {
            let value_at_coord = field[input_tuple.1 as usize][input_tuple.2 as usize];
            if value_at_coord == 9{
                map = change_map(coord, 12, map);
                break;
            }
            map = change_map(coord, value_at_coord, map);
        }
    }
    fieldprinter::print_map(&map);
    println!("Du bist auf eine Bombe gestoßen! Mehr Glück nächstes Mal.")
}


