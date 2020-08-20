// -*- coding:utf-8-unix -*-

#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // 0.2.2
extern crate num_traits; // 0.2.8
use num_bigint::BigInt;
use num_traits::Pow;

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use libm::*;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::*;
use std::ops::Range;
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

const MOD: usize = 1000000000 + 7;

//abc156-D
// #[fastout]
fn main() {
    input![n_seed: usize, a_f: usize, b: usize];

    // let mut v = 2;
    // loop {
    //     v *= 2;
    //     v = v % LargeNum;
    //     println!("{}", v);
    //     if v % LargeNum == 2 {
    //         // println!("{}: {}", v, v % LargeNum);
    //         break;
    //     }
    // }
    // eprintln!("usize.max: {}", std::usize::MAX);
    let multi_num = log2(n_seed as f64) as usize;

    let mut ans = 2;

    let mut n = n_seed;
    let mut a = 2;
    let mut x = 1;
    while n > 0 {
        eprintln!("n, x, a, n&1= {},{},{},{}", n, x, a, n & 1);
        //全てのbitが捨てられるまで。
        if n & 1 == 1 {
            //1番右のbitが1のとき。
            x = (x * a) % MOD;
        }
        a = a * a % MOD;
        n >>= 1; //bit全体を右に1つシフトして一番右を捨てる。
    }

    ans = x;
    eprintln!("2^{} = {},", n_seed, x);
    ans = (ans + MOD - 1) % MOD; //0本の組み合わせは抜く

    // let mut fac = vec![0; 200000 + 1];
    // let mut finv = vec![0; 200000 + 1];
    // let mut inv = vec![0; 200000 + 1];
    // fac[0] = 1;
    // fac[1] = 1;
    // finv[0] = 1;
    // finv[1] = 1;
    // inv[1] = 1;
    // for i in 2..200000 + 1 {
    //     fac[i] = fac[i - 1] * i % MOD;
    //     inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
    //     finv[i] = finv[i - 1] * inv[i] % MOD;
    // }

    // let com = |n, k| {
    //     if n < k {
    //         return 0;
    //     }
    //     // if n < 0 || k < 0 {
    //     //     return 0;
    //     // }

    //     return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD;
    // };

    // eprintln!("com({},{})={}", n_seed, a_f, com(n_seed, a_f));
    // eprintln!("com({},{})={}", n_seed, b, com(n_seed, b));
    // ans = ((ans + MOD) - com(n_seed, a_f)) % MOD;
    // ans = ((ans + MOD) - com(n_seed, b)) % MOD;

    // println!("{}", ans);
    // // // # n!^-1 の計算
    // // inv = pow(f, MOD - 2, MOD)
    // // // # n!^-1 - 1!^-1 の計算
    // // invs = [1] * (n + 1)
    // // invs[n] = inv
    // // for m in range(n, 1, -1):
    // //     inv *= m
    // //     inv %= MOD
    // //     invs[m - 1] = inv
    // // //ans - c_n_a - c_n_b;
}

// fn go(mid: usize, d: usize, use_num: usize, num753_cand: &mut Vec<usize>, n: usize) {}

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref VALUES: Mutex<Vec<i32>> = Mutex::default();
// }

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);
