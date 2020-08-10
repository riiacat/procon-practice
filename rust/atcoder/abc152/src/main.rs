// -*- coding:utf-8-unix -*-

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

//abc152-D
#[fastout]
fn main() {
    input![n: u64];

    let mut ans = 1;

    let digits = (log10(n as f64) + 1.0) as u64;

    let mut max_ditit_num = n;
    while max_ditit_num > 9 {
        max_ditit_num /= 10;
    }

    for d in 1..(digits + 1) {
        let mid = calc_conbination(1, d);
        ans = (max_ditit_num * mid + 9) * ans;
    }

    // ans = mid + ((mid * ans * max_ditit_num) as f64 / 9.0) as u64;

    println!("{}", ans);
}

// fn go(n: u64) -> u64 {
//     let digits = (log10(n as f64) + 1.0) as u64;

//     let mut ans = 0;
//     for digit in 1..digits {
//         ans += calc_conbination(10, digit);
//     }

//     let mut max_ditit_num = n;
//     while max_ditit_num > 9 {
//         max_ditit_num /= 10;
//     }

//     ans += calc_conbination(max_ditit_num, digits);

//     return ans;
// }

fn calc_conbination(_end: u64, digits: u64) -> u64 {
    // end: 1 -> 1, 2->2
    if digits <= 2 {
        return 1;
    }

    let multi = pow(10 as f64, (digits - 1) as f64) as u64;

    return multi;
}
