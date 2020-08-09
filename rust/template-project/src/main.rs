// -*- coding:utf-8-unix -*-

use proconio::input;
use std::io::*;
use std::str::FromStr;

// ABC086C - Traveling

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    // input!
    //     input! {
    //         n: u8,
    //         m: u8,
    //         sk: [[u8]; n],
    //         ps: [u8; m]
    //     }

    //     println!(
    //         r#"
    // n: {},
    // m: {},
    // sk: {:?},
    // ps: {:?}
    //         "#,
    //         n, m, sk, ps
    //     );

    // read
    // let n: usize = read();
    // let m: usize = read();
    // let mut s: Vec<Vec<usize>> = vec![vec![]; m];
    // let mut p: Vec<usize> = vec![];

    // for i in 0..m {
    //     let k = read();
    //     for _ in 0..k {
    //         s[i].push(read());
    //     }
    // }

    // for _ in 0..m {
    //     p.push(read());
    // }
    // let mut ans = 0;
}
