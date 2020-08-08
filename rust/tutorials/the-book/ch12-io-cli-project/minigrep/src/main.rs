use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("クエリ: {}", config.query);
    // println!("検索対象ファイル: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Appエラー: {}", e);
        process::exit(1);
    };
}
