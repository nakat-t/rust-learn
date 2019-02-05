extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数を当ててね");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("数字を入力: ");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("行の読み込みに失敗しました");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("あなたの予想: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("やったね！");
                break;
            }
        }
    }
}
