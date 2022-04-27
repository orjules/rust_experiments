use std::io::{self, Error, ErrorKind};

pub fn to_decimal(input:&String) -> i32{
    let mut sum: i32 = 0;
    let characters:Vec<char> = input.chars().collect();

    for i in 0..characters.len(){
        let value_this = match get_value_of(&characters[i]){
            Ok(val) => val,
            Err(error) => {
                panic!("Aborting because: {}", error);
            } 
        };

        let value_next = if i != characters.len()-1{
            match get_value_of(&characters[i+1]){
                Ok(val) => val,
                Err(error) => {
                    panic!("Aborting because: {}", error);
                }
            }
        } else {
            0
        };

        if value_this >= value_next{
            sum += value_this as i32;
        } else {
            sum -= value_this as i32;
        }
    }
    return sum;
}

fn get_value_of(c: &char) -> Result<u16, io::Error> {
    match c {
        'M' => Ok(1000),
        'D' => Ok(500),
        'C' => Ok(100),
        'L' => Ok(50),
        'X' => Ok(10),
        'V' => Ok(5),
        'I' => Ok(1),
        _ => return Err(Error::new(ErrorKind::InvalidInput, format!("Character {} is not a roman numeral!", c))), 
    }
}
