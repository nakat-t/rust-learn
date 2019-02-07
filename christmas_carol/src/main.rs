fn ordinal_number(n: u32) -> &'static str {
    match n {
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
        12 => "twelfth",
        _ => "?",
    }
}

fn main() {
    let lyric = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings, badam-pam-pam",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];
    
    println!("On the {} day of Christmas", ordinal_number(1));
    println!("My true love gave to me");
    println!("A partridge in a pear tree");
    
    for i in 2..=12 {
        println!("On the {} day of Christmas", ordinal_number(i));
        println!("My true love gave to me");
        for j in (2usize ..= i as usize).rev() {
            println!("{}", lyric[j-2]);
        }
        println!("And a partridge in a pear tree");
    }
}
