// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::collections::BinaryHeap;
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

//abc161-D
// #[fastout]
fn main() {
    input![n: usize];

    let mut rnrn_count = vec![vec![0; 64]; 10];

    // 1を除く
    for i in 1..10 {
        //先頭がiで 0+1桁のルンルン数
        rnrn_count[i][0] = 1 as u64;
    }

    for d in 1..15 {
        rnrn_count[0][d] = 2 * rnrn_count[0][d - 1];
        rnrn_count[9][d] = 2 * rnrn_count[9][d - 1];
        for a in 1..9 {
            rnrn_count[a][d] = 3 * rnrn_count[a][d - 1];
        }
    }

    let mut count = 0;
    let mut comsum_count = vec![0; 15];
    for d in 0..15 {
        let mut sum = 0;
        for a in 0..9 {
            sum += rnrn_count[a][d];
        }
        count += sum;
        comsum_count[d] = count;
        eprintln!("{} ditits rnrn=> {}, sum={}", d + 1, sum, count);
    }

    //decide digit
    let mut d = comsum_count.upper_bound(&(n as u64));
    let count = if d > 0 { comsum_count[d - 1] } else { 0 };
    let mut rest = n as u64 - count;
    // d+1桁目、 rest番目のrnrn数を求める必要がある

    let mut anss = Vec::new();
    eprintln!("{}, rest={}", d, rest);
    while (rest != 0) {
        let mut comsum_count_d_by1 = vec![0; 10];
        {
            let mut comsum_count = 0;
            for a in 0..10 {
                let sum = rnrn_count[a][d];
                comsum_count += sum;
                comsum_count_d_by1[a] = comsum_count;
            }
        }

        eprintln!("comsum_count_d_by1: {:?}", comsum_count_d_by1);
        let a = comsum_count_d_by1.upper_bound(&(rest));
        //a decided
        anss.push((a, d));
        eprintln!("{:?}", anss);
        rest -= comsum_count_d_by1[a - 1];
        d -= 1;
    }

    let ans: usize = anss
        .iter()
        .map(|(a, d)| *a * (10u64.pow(*d as u32) as usize))
        .fold(0, |mid, a| mid + a);

    println!("{}", ans);
    // let mut count = 0;
    // is_rnrn[0] = true;
    // for i in 1..10 {
    //     count += 1;
    //     eprintln!("{}'s rnrn: {}", count, i);
    //     if n == count {
    //         println!("{}", i);
    //         return;
    //     }
    //     is_rnrn[i] = true;
    // }

    // for i in 10..(1000000) {
    //     let d = log10(i as f64) as u32;
    //     let a = 10i64.pow(d);
    //     let max_d = i as i64 / a as i64;
    //     let no2_d = i as i64 / (10i64.pow(d - 1)) % 10 as i64;
    //     let rest_digits = i - max_d * a;
    //     // eprintln!("{}, {}, {}, {}", i, max_d, no2_d, rest_digits);
    //     if !is_rnrn[rest_digits as usize] {
    //         is_rnrn[i as usize] = false;
    //     } else if fabs((max_d - no2_d) as f64) as usize <= 1 {
    //         count += 1;
    //         eprintln!("{}'s rnrn: {}", count, i);
    //         if n == count {
    //             println!("{}", i);
    //             break;
    //         }
    //         is_rnrn[i as usize] = true;
    //     } else {
    //         is_rnrn[i as usize] = false;
    //     }
    // }
}

fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
