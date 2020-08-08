extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数字を推測してください");

    let sec_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("あなたの推測を入力してください");

        // println!("シークレットナンバーは: {}", sec_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み取りに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの推測は： {}", guess);

        match guess.cmp(&sec_number) {
            Ordering::Less => println!("小さすぎます！！"),
            Ordering::Greater => println!("大きすぎる。。"),
            Ordering::Equal => {
                println!("正解！！");
                break;
            }
        }
    }
}
