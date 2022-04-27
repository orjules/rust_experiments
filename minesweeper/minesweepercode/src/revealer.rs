
pub fn make_revealed_map(size: u16) -> Vec<Vec<char>>{
    let mut map = Vec::new();
    for _index1 in 0..size{
        let mut row = Vec::new();
        for _index2 in 0..size{
            row.push('?');
        }
        map.push(row);
    }
    return map;
}

pub fn is_marked(coord: (usize, usize), map: Vec<Vec<char>>) -> bool{
    if map[coord.0][coord.1] == 'X' {
        return true;
    }
    return false;
}

pub fn is_revealed(coord: (usize, usize), map: Vec<Vec<char>>) -> bool{
    if map[coord.0][coord.1] == '?' {
        return false;
    }
    return true;
}

pub fn add_mark_to_map(coord: (u16, u16), old_map: Vec<Vec<char>>) -> Vec<Vec<char>>{
    return change_map(coord, 10, old_map);
}

pub fn remove_mark_on_map(coord: (u16, u16), old_map: Vec<Vec<char>>) -> Vec<Vec<char>>{
    return change_map(coord, 11, old_map);
}

pub fn change_map(coord: (u16, u16), code: u16, old_map: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut map = old_map;
    map[coord.0 as usize][coord.1 as usize] = code_to_char(code);
    return map;
}

fn code_to_char(code: u16) -> char{
    match code {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'X',
        11 => '?',
        12 => 'B',
        _ => 'E'
    }
}