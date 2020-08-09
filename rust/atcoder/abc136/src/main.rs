// -*- coding:utf-8-unix -*-

use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
use std::convert::TryInto;
use std::io::*;
use std::str::FromStr;
use superslice::*;

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

//abc136-D
#[fastout]
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

    input![s: String,];

    let mut ans = vec![0; s.len()];

    //rlを探す
    let mut rl_idx = Vec::new();
    let mut previous_s = 'A';
    for (idx, s) in s.chars().enumerate() {
        if previous_s == 'R' && s == 'L' {
            rl_idx.push(idx - 1);
        }

        previous_s = s;
    }

    // eprintln!("{:?}", rl_idx);

    //rlまでの距離をはかって、どちらかに足す
    for (idx, r_or_l) in s.chars().enumerate() {
        if idx == 0 {
            if &s[0..2] == "RL" {
                ans[0] += 1;
            } else {
                //Rまでの距離を図る
                let r_idx = rl_idx[rl_idx.upper_bound(&(idx))];
                // eprintln!("R:{}, {}", idx, r_idx);
                let len = r_idx - idx;

                if len % 2 == 0 {
                    ans[r_idx] += 1;
                } else {
                    ans[r_idx + 1] += 1;
                }
            }
            continue;
        }
        if r_or_l == 'R' {
            //Rまでの距離を図る
            let r_idx = rl_idx[rl_idx.upper_bound(&(idx - 1))];
            // eprintln!("R:{}, {}", idx, r_idx);
            let len = r_idx - idx;

            if len % 2 == 0 {
                ans[r_idx] += 1;
            } else {
                ans[r_idx + 1] += 1;
            }
        } else {
            //Lまでの距離を図る
            let l_idx = rl_idx[rl_idx.lower_bound(&idx) - 1];
            // eprintln!("L:{}, {}", idx, l_idx);
            let len = (idx + 1) - l_idx;

            if len % 2 == 0 {
                ans[l_idx + 1] += 1;
            } else {
                ans[l_idx] += 1;
            }
        }
    }

    print!("{} ", ans[0]);

    for ans in &ans[1..] {
        print!("{} ", ans);
    }
}
