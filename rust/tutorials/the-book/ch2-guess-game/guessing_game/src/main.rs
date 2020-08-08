use std::io;

fn main() {
    println!("数字を推測してください");
    println!("あなたの推測を入力してください");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み取りに失敗しました");

    println!("あなたの推測は： {}", guess);
}
