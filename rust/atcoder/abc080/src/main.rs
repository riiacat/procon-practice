// -*- coding:utf-8-unix -*-

extern crate lazy_static;
extern crate num_bigint;
// 0.2.2
extern crate num_traits;

// 0.2.8
use num_bigint::BigInt;
use num_traits::{one, zero, Num, NumAssignOps, NumOps, One, Pow, Zero};

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
// use std::convert::TryInto;
use itertools::{assert_equal, concat};
use lazy_static::lazy_static;
// use libm::*;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::*;
use std::mem::swap;
use std::ops::{BitAnd, Range, ShrAssign};
use std::str::FromStr;
use std::sync::Mutex;
use superslice::*;

// ##########
// read
// ###########
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

// ##########
// chmin, chmax
// https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
// ###########

#[allow(unused_macros)]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

#[allow(unused_macros)]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

// ##########
// lazy_static!
// ##########
// lazy_static! {
//     static ref H: Mutex<Vec<i32>> = Mutex::default();
//     static ref W: Mutex<Vec<i32>> = Mutex::default();
// }
// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// const MOD: usize = 1000_000_000 + 7;

// ##########
// modint
// https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
// ##########

#[allow(dead_code)]
fn modinv<T: Num + NumAssignOps + NumOps + Copy + PartialOrd>(a: T, m: T) -> T {
    let mut a = a;
    let mut b = m;
    let mut u: T = one();
    let mut v: T = zero();

    while b != zero() {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    u %= m;
    if u < zero() {
        u += m;
    }
    return u;
}

#[test]
fn modinv_test() {
    assert_eq!(1, modinv(1, 13));
    assert_eq!(2, modinv(7, 13));
    assert_eq!(3, modinv(9, 13));
    assert_eq!(4, modinv(10, 13));
    assert_eq!(5, modinv(8, 13));
}

// long long modpow(long long a, long long n, long long mod) {
// long long res = 1;
// while (n > 0) {
// if (n & 1) res = res * a % mod;
// a = a * a % mod;
// n >>= 1;
// }
// return res;
// }
#[allow(dead_code)]
fn modpow<T>(a: T, n: T, modulo: T) -> T
where
    T: Num + NumAssignOps + NumOps + Copy + PartialOrd + BitAnd + PartialEq + ShrAssign,
    <T as BitAnd>::Output: PartialEq + Num,
{
    let mut res = one();
    let mut a = a;
    let mut n = n;
    while n > zero() {
        if (n & one()) == one() {
            res = res * a % modulo;
        }
        a = a * a % modulo;
        n >>= one();
    }

    return res;
}

#[test]
fn modpow_test() {
    assert_eq!(4, modpow(2, 2, 13));
    assert_eq!(3, modpow(2, 4, 13));
}

// 前処理 com_init(): O(n)
// クエリ処理 COM(n, k): O(1)
// conv::com_init();
// conv::com(n,k);
mod comb {
    use super::*;
    lazy_static! {
        static ref FAC: Mutex<Vec<usize>> = Mutex::default();
        static ref FINV: Mutex<Vec<usize>> = Mutex::default();
        static ref INV: Mutex<Vec<usize>> = Mutex::default();
        static ref MODULO: Mutex<usize> = Mutex::default();
        // static ref MAXNCONV: Mutex<usize> = Mutex::default();
    }

    // // テーブルを作る前処理
    // void com_init() {
    // fac[0] = fac[1] = 1;
    // finv[0] = finv[1] = 1;
    // inv[1] = 1;
    // for (int i = 2; i < MAX; i++){
    // fac[i] = fac[i - 1] * i % MOD;
    // inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
    // finv[i] = finv[i - 1] * inv[i] % MOD;
    // }

    #[allow(dead_code)]
    fn com_init_with(modulo: usize, maxn_conv: usize) {
        let mut fac = FAC.lock().unwrap();
        let mut finv = FINV.lock().unwrap();
        let mut inv = INV.lock().unwrap();
        *fac = vec![0; maxn_conv];
        *finv = vec![0; maxn_conv];
        *inv = vec![0; maxn_conv];

        let mut g_modulo = MODULO.lock().unwrap();
        *g_modulo = modulo;

        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;

        for i in 2..maxn_conv {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
    }

    #[allow(dead_code)]
    pub fn com_init() {
        com_init_with(MOD, MAXN_CONV);
    }

    // // 二項係数計算
    // long long COM(int n, int k){
    // if (n < k) return 0;
    // if (n < 0 || k < 0) return 0;
    // return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD;
    // }
    #[allow(dead_code)]
    pub fn com(n: usize, k: usize) -> usize {
        let fac = FAC.lock().unwrap();
        let finv = FINV.lock().unwrap();
        // let mut inv = INV.lock().unwrap();
        let m = *MODULO.lock().unwrap();

        if n < k {
            return 0;
        }
        // if n < 0 || k < 0 {
        //     return 0;
        // }
        return fac[n] * (finv[k] * finv[n - k] % m) % m;
    }

    #[test]
    fn com_test() {
        com_init_with(13, 100);
        assert_eq!(12, com(12, 1));
        assert_eq!(66 % 13, com(12, 2));
        assert_eq!(220 % 13, com(12, 3));
        assert_eq!(495 % 13, com(12, 4));
        assert_eq!(792 % 13, com(12, 5));
        assert_eq!(924 % 13, com(12, 6));
        assert_eq!(com(12, 5), com(12, 7));
    }
}

// ##########
// union-find
// http://sntea.hatenablog.com/entry/2017/06/07/091246
// ##########

mod uf {
    // let mut uf = uf::UnionFind::new(10);
    // uf.unite; uf.same
    #[allow(dead_code)]
    struct UnionFind {
        par: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        #[allow(dead_code)]
        pub fn new(n: usize) -> UnionFind {
            let mut vec = vec![0; n];
            for i in 0..n {
                vec[i] = i;
            }
            UnionFind {
                par: vec,
                rank: vec![0; n],
            }
        }

        #[allow(dead_code)]
        fn find(&mut self, x: usize) -> usize {
            if x == self.par[x] {
                x
            } else {
                let par = self.par[x];
                let res = self.find(par);
                self.par[x] = res;
                res
            }
        }

        #[allow(dead_code)]
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }

        #[allow(dead_code)]
        pub fn unite(&mut self, a: usize, b: usize) {
            let apar = self.find(a);
            let bpar = self.find(b);
            if self.rank[apar] > self.rank[bpar] {
                self.par[bpar] = apar;
            } else {
                self.par[apar] = bpar;
                if self.rank[apar] == self.rank[bpar] {
                    self.rank[bpar] += 1;
                }
            }
        }
    }

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::new(10);

        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(i == j, uf.same(i, j));
            }
        }

        uf.unite(0, 1);
        assert_eq!(true, uf.same(0, 1));
        //false
        assert_eq!(false, uf.same(0, 9));
        assert_eq!(false, uf.same(1, 9));
        assert_eq!(false, uf.same(2, 9));

        uf.unite(8, 9);
        assert_eq!(true, uf.same(0, 1));
        assert_eq!(true, uf.same(8, 9));
        //false
        assert_eq!(false, uf.same(0, 9));
        assert_eq!(false, uf.same(1, 9));
        assert_eq!(false, uf.same(2, 9));

        uf.unite(1, 9);
        assert_eq!(true, uf.same(0, 1));
        assert_eq!(true, uf.same(8, 9));
        assert_eq!(true, uf.same(0, 8));
        assert_eq!(true, uf.same(0, 9));
        assert_eq!(true, uf.same(1, 8));
        assert_eq!(true, uf.same(1, 9));
        //false
        assert_eq!(false, uf.same(2, 9));
    }
}

// ###########
// seg_tree
// ant_book
// ###########
mod seg_tree {
    #[derive(Debug)]
    struct SegTree<T: Clone> {
        n: usize,
        dat: Vec<Option<T>>,
    }

    impl<T: Clone + std::fmt::Debug> SegTree<T> {
        #[allow(dead_code)]
        pub fn new(size: usize) -> SegTree<T> {
            let mut size_pow2 = 1;
            while size_pow2 < size {
                size_pow2 *= 2;
            }

            let dat: Vec<Option<T>> = vec![None; 2 * size_pow2 - 1];
            SegTree { n: size_pow2, dat }
        }

        #[allow(dead_code)]
        pub fn update<F: Fn(&Option<T>, &Option<T>) -> Option<T>>(
            &mut self,
            k: usize,
            a: T,
            update: F,
        ) {
            let mut k = k;
            k += self.n - 1;
            self.dat[k] = Some(a);
            while k > 0 {
                k = (k - 1) / 2;
                self.dat[k] = update(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
            }
        }

        #[allow(dead_code)]
        fn query_inner<F: Fn(&Option<T>, &Option<T>) -> Option<T>>(
            &self,
            selection_query: &F,
            a: usize,
            b: usize,
            k: usize,
            l: usize,
            r: usize,
        ) -> Option<T> {
            if r <= a || b <= l {
                // eprintln!("{}, {}, {}, {}, {:?}", a, b, l, r, "none");
                return None;
            }

            return if a <= l && r <= b {
                // eprintln!("{}, {}, {}, {}, {:?}", a, b, r, l, self.dat[k]);
                self.dat[k].clone()
            } else {
                let vl = self.query_inner(selection_query, a, b, k * 2 + 1, l, (l + r) / 2);
                let vr = self.query_inner(selection_query, a, b, k * 2 + 2, (l + r) / 2, r);

                selection_query(&vl, &vr)
            };
        }

        #[allow(dead_code)]
        pub fn query<F: Fn(&Option<T>, &Option<T>) -> Option<T>>(
            &self,
            selection_query: &F,
            a: usize,
            b: usize,
        ) -> Option<T> {
            return self.query_inner(selection_query, a, b, 0, 0, self.n);
        }
    }

    #[test]
    fn test_segtree_rmq() {
        let mut t: SegTree<usize> = SegTree::new(5);

        let cmp_f = |lhs: &Option<usize>, rhs: &Option<usize>| {
            if lhs.is_none() {
                return rhs.clone();
            }

            if rhs.is_none() {
                return lhs.clone();
            }

            return if lhs.unwrap() <= rhs.unwrap() {
                lhs.clone()
            } else {
                rhs.clone()
            };
        };
        // 1, 3, 2, 5, 1
        t.update(0, 1, cmp_f);
        t.update(1, 3, cmp_f);
        t.update(2, 2, cmp_f);
        t.update(3, 5, cmp_f);
        t.update(4, 1, cmp_f);
        // println!("{:?}", t);

        assert_eq!(1, t.query(&cmp_f, 0, 1).unwrap());
        assert_eq!(3, t.query(&cmp_f, 1, 2).unwrap());
        assert_eq!(2, t.query(&cmp_f, 2, 3).unwrap());
        assert_eq!(5, t.query(&cmp_f, 3, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 4, 5).unwrap());

        // len2
        assert_eq!(1, t.query(&cmp_f, 0, 2).unwrap());
        assert_eq!(2, t.query(&cmp_f, 1, 3).unwrap());
        assert_eq!(2, t.query(&cmp_f, 2, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 3, 5).unwrap());

        // len3
        assert_eq!(1, t.query(&cmp_f, 0, 3).unwrap());
        assert_eq!(2, t.query(&cmp_f, 1, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 2, 5).unwrap());

        // len4
        assert_eq!(1, t.query(&cmp_f, 0, 4).unwrap());
        assert_eq!(1, t.query(&cmp_f, 1, 5).unwrap());

        // len5
        assert_eq!(1, t.query(&cmp_f, 0, 6).unwrap());
    }
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);

// MOD, Combination関連に使う定数
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
const MAXN_CONV: usize = 510000;

// abc080-C
// #[fastout]
fn main() {
    const TIMES: usize = 10;

    input![n: usize, f: [[i8; TIMES]; n], p: [[i64; TIMES + 1]; n]];
    // eprintln!("{:?}", f);
    // eprintln!("{:?}", p);

    let mut ans = std::i64::MIN / 2;
    for i in 1..2usize.pow(TIMES as u32) {
        let mut opens = vec![false; TIMES];

        let mut idx = 0;
        let mut i = i;
        while i > 0 {
            if i & 1 == 1 {
                opens[idx] = true;
            }

            i >>= 1;
            idx += 1;
        }

        let mut cand: i64 = 0;

        for i in 0..n {
            let same_open: usize = f[i]
                .iter()
                .zip(opens.iter())
                .map(|(b1, b2)| if (*b1 == 1) && *b2 { 1 } else { 0 })
                .sum();

            // eprintln!(
            //     "{}, {:?}, {}, {}, {}",
            //     cand, opens, i, same_open, p[i][same_open]
            // );
            cand += p[i][same_open];
        }

        chmax!(ans, cand);
    }

    println!("{}", ans);
}
