pub fn run() {
    let things : [&str; 12] = [
        "a partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"];

    for num in 1..13{
        println!("On the {} day of christmas, my true love gave to me", num_to_str(num));
        for index in (0..num).rev(){
            println!(" {}", things[index as usize]);
            if index == 1{
                print!(" And");
            }
        }
    }
}

fn num_to_str(num: u32) -> &'static str{
    match num {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        _ => "twelfth"
    }
}