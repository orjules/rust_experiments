use rand::Rng;

pub fn change_bomb_on_field(coord: (usize, usize), field: & mut Vec<Vec<u16>>) { 
    if field[coord.0][coord.1] == 9 { // Wenn Bombe (9) -> zu "gefundene" Bombe (10) machen
        field[coord.0].insert(coord.1, 10);
        return new_field;
    }else if field[coord.0][coord.1] == 10 { // Wenn "gefundene" Bombe (10) -> zu Bombe machen (9)
        field[coord.0].insert(coord.1, 9);
        return new_field;
    }
}

pub fn is_done(field: Vec<Vec<u16>>) -> bool{
    for row in 0..field.len() {
        if field[row].contains(&9){
            return false;
        }
    }
    return true;
}

pub fn get_field(size: u16, bomb_count: u16) -> Vec<Vec<u16>>{
    // Feld mit 0 überall initialisieren
    let mut field = Vec::new();
    for _num1 in 0..size{
        let mut x:Vec<u16> = Vec::new();
        for _num2 in 0..size{
            x.push(0);
        }
        field.push(x);
    }
    let bombs = get_bomb_coord(size, bomb_count);
    field = define_numbers(&bombs, field);
    return field;
}

fn get_bomb_coord(size: u16, bomb_count: u16) -> Vec<(u16, u16)>{
    let mut bomb_list = Vec::new();
    for _n in 0..bomb_count{                     //Jetzt bekomme ich doch immer bomb_count-1 bomben
        loop {
            let num_x = rand::thread_rng().gen_range(0..size);
            let num_y = rand::thread_rng().gen_range(0..size);
            let tuple = (num_y, num_x);
            if !bomb_list.contains(&tuple){
                bomb_list.push(tuple);
                break;
            }else {
                continue
            }
        }
    }
    // debug_bombs(&bomb_list);
    return bomb_list;
}

fn define_numbers(bomb_list: &Vec<(u16, u16)>, field: Vec<Vec<u16>>) -> Vec<Vec<u16>>{
    // TODO kann ich dem hier nich eine referenz vom Feld geben?
    let size = field.len() as u16;
    let mut new_field = field;

    for bomb in bomb_list {
        let neighbours = get_neighbours(bomb, size as i16);
        for neighbour in neighbours {
            if !bomb_list.contains(&neighbour){
                new_field[neighbour.0 as usize][neighbour.1 as usize] += 1;
            }
        }
        new_field[bomb.0 as usize][bomb.1 as usize] = 9; //ist die interne Bezeichnung für Bombe
    }
    return new_field;
}

fn get_neighbours(coord: &(u16, u16), size: i16) -> Vec<(u16, u16)>{
    //  TODO coord ist ein dummer name, müsste bomb_coord oder so sein
    let mut neighbours = Vec::new();
    for y in 0..3{
        for x in 0..3 {
            let nei_y = (coord.0 + y) as i16 - 1;
            let nei_x = (coord.1 + x) as i16 - 1;
            if nei_x >= 0 && nei_x < size && nei_y >= 0 && nei_y < size{
                if nei_x != coord.1 as i16 || nei_y != coord.0 as i16{
                    neighbours.push((nei_y as u16, nei_x as u16))
                }
            }
        }
    }
    // debug_neighbours(coord, &neighbours);
    return neighbours;
}

/*
fn debug_bombs(bombs: &Vec<(u16, u16)>){
    println!("Bombenpositionen: ");
    for bomb in bombs {
        println!("Bombe bei: {}, {}", bomb.0, bomb.1);
    }
}

fn debug_neighbours(coord: &(u16, u16), nei: &Vec<(u16, u16)>){
    println!("Die Nachbarn von {}, {} sind: ", coord.0, coord.1);
    for n in nei {
        println!("Nachbar bei: {}, {}", n.0, n.1);
    }
}
 */
