// -*- coding:utf-8-unix -*-

use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
use std::convert::TryInto;
use std::io::*;
use std::str::FromStr;

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

//abc128-C

#[fastout]
fn main() {
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

    let n: usize = read();
    let m: usize = read();
    let mut s: Vec<Vec<usize>> = vec![vec![]; m];
    let mut p: Vec<usize> = vec![];

    for i in 0..m {
        let k = read();
        for _ in 0..k {
            s[i].push(read());
        }
    }

    for _ in 0..m {
        p.push(read());
    }
    let mut ans = 0;

    let mut sstate = vec![false; n];

    for switch_state in 0..(2u64.pow(n.try_into().unwrap())) {
        for i in 0..n {
            sstate[i] = (switch_state >> i) % 2 == 0
        }

        let mut is_ok = true;
        for i in 0..m {
            let mut sum = 0;
            for s in &s[i] {
                sum += if sstate[s - 1] { 1 } else { 0 }
            }

            if sum % 2 != p[i] {
                is_ok = false;
            }
        }

        if is_ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
