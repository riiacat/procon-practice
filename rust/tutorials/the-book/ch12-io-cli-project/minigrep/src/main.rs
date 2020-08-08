use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("クエリ: {}", query);
    println!("検索対象ファイル: {}", filename);

    let contents =
        fs::read_to_string(filename).expect("ファイルの読み込み中にエラーが発生しました。");

    println!("ファイルコンテンツ：\n{}", contents);
}
