// -*- coding:utf-8-unix -*-

extern crate lazy_static;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
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

//abc114-C
// #[fastout]
fn main() {
    input![n: usize];

    let mut is_num7 = vec![0; n + 1];
    let mut is_num5 = vec![0; n + 1];
    let mut is_num3 = vec![0; n + 1];

    let mut ans = 0;

    // eprintln!("{}", n);
    for i in 1..(n + 1) {
        if i < 10 {
            if i != 7 && i != 5 && i != 3 {
            } else {
                match i {
                    3 => is_num3[i] = 1,
                    5 => is_num5[i] = 1,
                    7 => is_num7[i] = 1,
                    _ => panic!("not"),
                }
            }
        } else {
            let mut head_ditit = i;
            let mut d = 0;
            while head_ditit >= 10 {
                head_ditit /= 10;
                d += 1;
            }

            let rest = i - head_ditit * 10usize.pow(d);
            is_num3[i] = is_num3[rest];
            is_num5[i] = is_num5[rest];
            is_num7[i] = is_num7[rest];

            match head_ditit {
                3 => is_num3[i] += 1,
                5 => is_num5[i] += 1,
                7 => is_num7[i] += 1,
                _ => {}
            }

            if d + 1 >= 3
                && is_num3[i] > 0
                && is_num5[i] > 0
                && is_num7[i] > 0
                && (is_num3[i] + is_num5[i] + is_num7[i] == d + 1)
            {
                // eprintln!(
                //     "{}, {}d, {}, {}, {}",
                //     i,
                //     d + 1,
                //     is_num3[i],
                //     is_num5[i],
                //     is_num7[i]
                // );
                ans += 1;
            }
        }
    }
    // eprintln!("{}", std::usize::MAX);

    println!("{}", ans);
}
