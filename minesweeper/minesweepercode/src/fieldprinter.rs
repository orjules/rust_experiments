pub fn print_real(field: &Vec<Vec<u16>>){
    let mut row_counter:u16 = 0;
    println!("Real Map");
    print_top_row(field.len() as u16);
    for row in field {
        let mut row_content = String::new();
        row_content.push_str(&format!("{} | ", row_counter));
        for column in row{
            row_content.push_str(&format!("{}  ", column));
        }
        println!("{}",row_content);
        row_counter += 1;
    }
}

pub fn print_map(field: &Vec<Vec<char>>){
    let mut row_counter:u16 = 0;
    println!("Game Map");
    print_top_row(field.len() as u16);
    for row in field {
        let mut row_content = String::new();
        row_content.push_str(&format!("{} | ", row_counter));
        for column in row{
            row_content.push_str(&format!("{}  ", column));
        }
        println!("{}",row_content);
        row_counter += 1;
    }
}

fn print_top_row(col_count:u16){
    let mut row_content_1 = String::new();
    row_content_1.push_str("  | ");
    for number in 0..col_count{
        row_content_1.push_str(&format!("{}  ", number));
    }
    println!("{}", row_content_1);
    let mut row_content_2 = String::new();
    row_content_2.push_str("==|=");
    for _number in 0..col_count-1{
        row_content_2.push_str("===");
    }
    row_content_2.push_str("==");
    println!("{}", row_content_2);
}