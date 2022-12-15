// -*- coding:utf-8-unix -*-

extern crate lazy_static;
extern crate num_bigint;
// 0.2.2
extern crate num_traits;

use itertools::enumerate;
use itertools::Itertools;
// 0.2.8
use num_bigint::BigInt;
use num_traits::{one, zero, Num, NumAssignOps, NumOps, One, Pow, ToPrimitive, Zero};

// use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
// use std::convert::TryInto;
use itertools::{assert_equal, concat};
use lazy_static::lazy_static;
// use libm::*;
use ascii::AsciiChar;
use core::time;
use rand::Rng;
use std::borrow::Borrow;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::*;
use std::mem::swap;
use std::ops::{BitAnd, Neg, Range, ShrAssign};
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
// modint
// https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
// ##########

#[allow(dead_code)]
fn modinv<T: Num + NumAssignOps + NumOps + Copy + PartialOrd + Neg>(a: T, m: T) -> T {
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
    #[derive(Debug)]
    pub struct UnionFind {
        par: Vec<i64>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        #[allow(dead_code)]
        pub fn new(n: usize) -> UnionFind {
            let mut vec = vec![0; n];
            for i in 0..n {
                vec[i] = -1;
            }
            UnionFind {
                par: vec,
                rank: vec![0; n],
            }
        }

        #[allow(dead_code)]
        fn find(&mut self, x: usize) -> usize {
            if self.par[x] < 0 {
                x
            } else {
                let par = self.par[x];
                let res = self.find(par as usize);
                self.par[x] = res as i64;
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
                self.par[apar] += self.par[bpar];
                self.par[bpar] = apar as i64;
            } else {
                self.par[bpar] += self.par[apar];
                self.par[apar] = bpar as i64;
                if self.rank[apar] == self.rank[bpar] {
                    self.rank[bpar] += 1;
                }
            }
        }

        #[allow(dead_code)]
        pub fn size(&mut self, x: usize) -> usize {
            let parent = self.find(x);
            //parentのparにサイズが負の状態で入る
            return (-self.par[parent]) as usize;
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

        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        //1
        assert_eq!(1, uf.size(2));
        assert_eq!(1, uf.size(8));
        assert_eq!(1, uf.size(9));

        uf.unite(8, 9);
        assert_eq!(true, uf.same(0, 1));
        assert_eq!(true, uf.same(8, 9));
        //false
        assert_eq!(false, uf.same(0, 9));
        assert_eq!(false, uf.same(1, 9));
        assert_eq!(false, uf.same(2, 9));

        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        assert_eq!(2, uf.size(8));
        assert_eq!(2, uf.size(9));
        //1
        assert_eq!(1, uf.size(2));

        uf.unite(1, 9);
        assert_eq!(true, uf.same(0, 1));
        assert_eq!(true, uf.same(8, 9));
        assert_eq!(true, uf.same(0, 8));
        assert_eq!(true, uf.same(0, 9));
        assert_eq!(true, uf.same(1, 8));
        assert_eq!(true, uf.same(1, 9));
        //false
        assert_eq!(false, uf.same(2, 9));

        assert_eq!(4, uf.size(0));
        assert_eq!(4, uf.size(1));
        assert_eq!(4, uf.size(8));
        assert_eq!(4, uf.size(9));
        //1
        assert_eq!(1, uf.size(2));
    }
}

// ###########
// seg_tree
// ant_book
// ###########
mod seg_tree {
    #[derive(Debug)]
    pub struct SegTree<T: Clone> {
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

// ##############
// rolling hash
// ###############
mod rolling_hash {
    use super::*;
    use ascii::{AsciiStr, AsciiString};
    use num_traits::AsPrimitive;

    fn contains_with(base: u64, a: &AsciiStr, b: &AsciiStr) -> bool {
        let (al, bl) = (a.len(), b.len());
        if al > bl {
            return false;
        }

        let mut t: u64 = 1;
        for _ in 0..al {
            t = t.wrapping_mul(base);
        }

        let (mut ah, mut bh): (u64, u64) = (0, 0);

        for i in 0..al {
            ah = ah.wrapping_mul(base) + a[i].as_byte() as u64;
        }
        for i in 0..al {
            bh = bh.wrapping_mul(base) + b[i].as_byte() as u64;
        }

        // eprintln!("{}, {}", ah, bh);

        for i in 0..=bl - al {
            if ah == bh {
                return true;
            }
            if i + al < bl {
                let mut add: i64 = b[i + al].as_byte().as_();
                add -= ((b[i].as_byte() as u64).wrapping_mul(t)) as i64;
                bh = (bh.wrapping_mul(base) as i64).wrapping_add(add) as u64;
            }
        }

        return false;
    }

    #[allow(dead_code)]
    pub fn contains(a: &AsciiStr, b: &AsciiStr) -> bool {
        return contains_with(BASE_ROLLING_HASH, a, b);
    }

    #[test]
    fn contains_test() {
        const base: u64 = 1000_000_007;
        assert_eq!(
            false,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("a").unwrap(),
            )
        );

        assert_eq!(
            true,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("aaabca").unwrap(),
            )
        );

        assert_eq!(
            true,
            contains_with(
                base,
                &AsciiString::from_str("aaaaaa").unwrap(),
                &AsciiString::from_str("aaaaaa").unwrap(),
            )
        );

        assert_eq!(
            false,
            contains_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("aacbaa").unwrap(),
            )
        )
    }

    fn overlap_last_and_head_with(base: u64, a: &AsciiStr, b: &AsciiStr) -> usize {
        let (al, bl) = (a.len(), b.len());

        let mut ans = 0;
        let (mut ah, mut bh, mut t): (u64, u64, u64) = (0, 0, 1);
        for i in 1..=min(al, bl) {
            ah = ah.wrapping_add((a[al - i].as_byte() as u64).wrapping_mul(t));
            bh = bh
                .wrapping_mul(base)
                .wrapping_add(b[i - 1].as_byte() as u64);

            if ah == bh {
                ans = i;
            }
            t = t.wrapping_mul(base);
        }

        return ans;
    }

    fn overlap_last_and_head(a: &AsciiStr, b: &AsciiStr) -> usize {
        return overlap_last_and_head_with(BASE_ROLLING_HASH, a, b);
    }

    #[test]
    fn overlap_test() {
        const base: u64 = 1000_000_007;
        assert_eq!(
            0,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("a").unwrap(),
            )
        );

        assert_eq!(
            2,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("abc").unwrap(),
                &AsciiString::from_str("bca").unwrap(),
            )
        );

        assert_eq!(
            5,
            overlap_last_and_head_with(
                base,
                &AsciiString::from_str("hogefoobar").unwrap(),
                &AsciiString::from_str("oobarhoge").unwrap(),
            )
        );
    }
}

#[allow(dead_code)]
fn to_alphabet_num(a: AsciiChar) -> usize {
    (a.as_byte() - AsciiChar::a.as_byte()) as usize
}

#[allow(dead_code)]
fn num_to_alphabet(a: usize) -> Option<AsciiChar> {
    let a = a
        .to_u8()
        .map(|a| AsciiChar::from_ascii(AsciiChar::a.as_byte() + a as u8).ok());
    return a.flatten();
}

// parse_text
#[macro_use]
pub mod parse_text {
    #[macro_export]
    macro_rules! input_text {
  ($s:expr=>$($t:tt)*) => {
    let mut lines=$s.split("\n");
    $(
        line_parse!(lines,$t);
    )*
  };
}

    macro_rules! line_parse {
  ($lines:expr,($($name:ident:$t:tt)*)) => {
    let mut line=$lines.next().unwrap().split_whitespace();
    $(value_def!(line,$name,$t);)*
  };

  //複数行
  ($lines:expr,{$n:expr;$name:ident:$t:tt}) => {
    values_def!($lines,$n,$name,$t);
  };
}

    macro_rules! value_def {
        ($line:expr, $name:ident, $t:tt) => {
            let $name = value!($line, $t);
        };
    }

    macro_rules! values_def {
        ($lines:expr, $n:expr, $name:ident, $t:tt) => {
            let $name = {
                let mut vec = Vec::new();
                for i in 0..$n {
                    let mut next = $lines.next().unwrap().split_whitespace();
                    vec.push(value!(next, $t));
                }
                vec
            };
        };
    }

    macro_rules! value {
  //配列
  ($line:expr,[$t:tt]) => {
    $line.map(|x|{
      let mut iter=::std::iter::once(x);
      value!(iter,$t)
    }).collect::<Vec<_>>()
  };
  //タプル
  ($line:expr,($($t:tt),*)) => {
    ($(value!($line,$t),)*)
  };
  //文字列
  ($line:expr,#) => {
    $line.next().unwrap()
  };
  //インデックス(-1)
  ($line:expr,@) => {
    $line.next().unwrap().parse::<usize>().unwrap()-1
  };
  //単一値
  ($line:expr,$t:ty) => {
    $line.next().unwrap().parse::<$t>().unwrap()
  };
}

    #[test]
    fn test1() {
        {
            input_text!(
              "3
5 2
2 3 4 5 6
10
20
30
1 2
8 1
2 3
4 1
1283 23 43 32
1 2 3
2 3 4
3 4 5
"=>
              (n:usize) //単一値
              (k:i64 p:i64) //複数値
              (list1:[i64]) //配列
              {n;list2:i64} //N回繰り返し
              (tup:(i64,i64)) //タプル
              {n;list3:(i64,i64)}
              (i:i64 list4:[i64])
              {n;map:[i64]}
            );
            assert_eq!(n, 3);
            assert_eq!(k, 5);
            assert_eq!(p, 2);
            assert_eq!(list1, vec![2, 3, 4, 5, 6]);
            assert_eq!(list2, vec![10, 20, 30]);
            assert_eq!(tup, (1, 2));
            assert_eq!(list3, vec![(8, 1), (2, 3), (4, 1)]);
            assert_eq!(i, 1283);
            assert_eq!(list4, vec![23, 43, 32]);
            assert_eq!(map, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        }

        {
            input_text!(
              "3
5 2
2 3 4 5 6
10
20
30
1 2
8 1
2 3
4 1
1283 23 43 32
1 2
"=>
              (n:usize) //単一値
              (k:# p:#) //複数値
              (list1:[#]) //配列
              {n;list2:#} //N回繰り返し
              (tup:(#,#)) //タプル
              {n;list3:(#,#)}
              (i:# list4:[#])
              (index:[@])
            );
            assert_eq!(n, 3);
            assert_eq!(k, "5");
            assert_eq!(p, "2");
            assert_eq!(list1, vec!["2", "3", "4", "5", "6"]);
            assert_eq!(list2, vec!["10", "20", "30"]);
            assert_eq!(tup, ("1", "2"));
            assert_eq!(list3, vec![("8", "1"), ("2", "3"), ("4", "1")]);
            assert_eq!(i, "1283");
            assert_eq!(list4, vec!["23", "43", "32"]);
            assert_eq!(index, vec![0, 1]);
        }
    }
}

// ##########
// Timer
// ##########
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Timer(Instant);

impl Timer {
    fn new() -> Timer {
        Timer(Instant::now())
    }

    fn elapsed(&self) -> f64 {
        let elapsed = self.0.elapsed();
        (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64) / 1_000_000_000.0
    }
}

// ##########
// lazy_static!
// ##########
lazy_static! {
    static ref D: Mutex<i64> = Mutex::default();
    static ref C: Mutex<Vec<i64>> = Mutex::default();
    static ref S: Mutex<Vec<Vec<i64>>> = Mutex::default();
    static ref TIMER: Mutex<Option<Timer>> = Mutex::default();
}

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);
// assert_eq!(&*values, &[1, 2, 3, 4]);

// let mut values = VALUES.lock().unwrap();
// values.extend_from_slice(&[1, 2, 3, 4]);

// MOD, Combination関連に使う定数
#[allow(dead_code)]
const BASE_ROLLING_HASH: u64 = 1158187049;
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
const MAXN_CONV: usize = 510000;

#[derive(Debug)]
struct State {
    selected: Vec<usize>,
    select_day: Vec<Vec<usize>>,
    score: Option<i64>,
}

impl State {
    fn new(days: i32) -> Self {
        let selected: Vec<usize> = vec![];
        let mut select_day: Vec<Vec<usize>> = vec![];

        for _ in 0..26 {
            select_day.push(vec![]);
        }

        State {
            selected,
            select_day,
            score: Option::None,
        }
    }

    fn interval_cost(&self, c: i64, interval: i64) -> i64 {
        let a = c * (interval - 1) as i64 * ((interval) as i64) / 2;
        return a;
    }

    fn calc_score(&mut self) -> i64 {
        let d = *D.lock().unwrap();
        let c = C.lock().unwrap();
        let s = S.lock().unwrap();

        let mut res: i64 = 0;
        for (i_day, sel_con) in enumerate(self.selected.iter()) {
            let a = s[i_day][*sel_con];
            // eprintln!("day: {}, sel_con: {}, a: {}", i_day, sel_con, a);
            res += a;
        }

        for con in 0..26 {
            let select_days = &self.select_day[con];
            for i in 0..(select_days.len()) {
                let old_day = if i == 0 { 0 } else { select_days[i - 1] };
                let interval = select_days[i] - old_day;
                if interval > 1 {
                    res -= self.interval_cost(c[con], interval as i64);
                }
            }

            let old_day = if (select_days.len() > 0) {
                select_days[select_days.len() - 1]
            } else {
                0
            };
            let interval = d + 1 - old_day as i64;
            res -= self.interval_cost(c[con], interval);
        }

        // eprintln!("score: {}", res);
        self.score = Some(res);

        return res;
    }

    fn print_out(&self) {
        for d in self.selected.iter() {
            println!("{}", d + 1);
        }
    }

    fn get_score(&self) -> i64 {
        self.score.unwrap()
    }

    fn make_greedy(&mut self, cost_interval: i64) {
        let d = *D.lock().unwrap();
        let c = C.lock().unwrap();
        let s = S.lock().unwrap();

        // 365 // 26 = 14くらいはコストがあると思う
        let s_in_c: Vec<Vec<i64>> = s
            .iter()
            .map(|s_d| {
                s_d.iter()
                    .zip(c.iter())
                    .map(|(s_con, c_con)| s_con + c_con * cost_interval)
                    .collect()
            })
            .collect();

        let mut selected_con = vec![];
        let mut selected_days: Vec<Vec<usize>> = (0..26).into_iter().map(|i| vec![]).collect_vec();
        for (i_d, s_in_c_d) in enumerate(s_in_c.iter()) {
            let (max_i, _) = s_in_c_d
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            selected_con.push(max_i);
            selected_days[max_i].push(i_d + 1);
        }

        self.selected = selected_con;
        self.select_day = selected_days;
    }

    fn change_and_rescore(&mut self, day: i64, new_con: usize) -> i64 {
        let mut score: i64;
        {
            let d = *D.lock().unwrap();
            let c = C.lock().unwrap();
            let s = S.lock().unwrap();

            score = if self.score.is_none() {
                // let mut score = if self.score.is_none() {
                self.calc_score()
            } else {
                self.get_score()
            };

            // eprintln!("{:?}", self);

            let old_con = self.selected[day as usize - 1];
            if old_con == new_con {
                return score;
            }
            self.selected[day as usize - 1] = new_con as usize;

            let old_con_day_pos = self.select_day[old_con]
                .iter()
                .position(|d| *d == (day as usize))
                .unwrap();
            let prev = old_con_day_pos as i64 - 1;
            let prev = if prev < 0 {
                0
            } else {
                self.select_day[old_con][prev as usize]
            };

            let next = old_con_day_pos + 1;
            let next = if next >= self.select_day[old_con].len() {
                d as usize + 1
            } else {
                self.select_day[old_con][next as usize]
            };

            let del_cost = self.interval_cost(c[old_con], next as i64 - prev as i64)
                - self.interval_cost(c[old_con], next as i64 - day)
                - self.interval_cost(c[old_con], day - prev as i64);
            score -= del_cost;
            score -= s[day as usize - 1][old_con];

            self.select_day[old_con].remove(old_con_day_pos);

            let new_con_day_pos = self.select_day[new_con as usize]
                .iter()
                .position(|a| *a > day as usize)
                .unwrap_or(self.select_day[new_con as usize].len());

            self.select_day[new_con as usize].insert(new_con_day_pos, day as usize);
            let prev = new_con_day_pos as i64 - 1;
            let prev = if prev < 0 {
                0
            } else {
                self.select_day[new_con][prev as usize]
            };

            let next = new_con_day_pos + 1;
            let next = if next >= self.select_day[new_con].len() {
                d as usize + 1
            } else {
                self.select_day[new_con][next as usize]
            };

            let del_cost = self.interval_cost(c[new_con], next as i64 - prev as i64)
                - self.interval_cost(c[new_con], next as i64 - day)
                - self.interval_cost(c[new_con], day - prev as i64);
            score += del_cost;
            score += s[day as usize - 1][new_con];
        }
        // eprintln!("asserting");
        // assert_eq!(self.calc_score(), score);
        // eprintln!("asserting done");

        self.score = Some(score);
        return score;
    }

    fn annealing_update(&mut self) {
        let timer = (*TIMER.lock().unwrap()).unwrap();
        let lim_d = *D.lock().unwrap();

        const T0: f64 = 2e3;
        const T1: f64 = 6e3;
        const TL_S: f64 = 2.0 - 0.05;
        const LOOP: usize = 1e6 as usize * 5;

        let mut rng = rand_pcg::Pcg64Mcg::new(890482);
        let mut T = 0 as f64;
        let mut i = 0;
        loop {
            // let old_score = self.calc_score();
            let old_score = self.get_score();
            if timer.elapsed() >= TL_S {
                break;
            }
            if i % 100 == 0 {
                let t = timer.elapsed() / TL_S;
                T = T0.powf(1.0 - t) * T1.powf(t);
            }

            if i % 100000 == 0 {
                eprintln!("i = {}, s={}, T={}, e={}", i, old_score, T, timer.elapsed());
            }

            if rng.gen_bool(0.5) {
                //random one point change
                let d: i64 = rng.gen_range(1, lim_d + 1);
                let con: usize = rng.gen_range(0, 26);
                let old_con = self.selected[d as usize - 1];

                let new_score = self.change_and_rescore(d, con);
                if new_score < old_score
                    && (!rng.gen_bool(f64::exp((new_score - old_score) as f64 / T)))
                {
                    self.change_and_rescore(d, old_con);
                } else {
                    // eprintln!(
                    //     "c: i={}, {}->{} ({}), T={}",
                    //     i,
                    //     old_score,
                    //     new_score,
                    //     new_score - old_score,
                    //     T
                    // );
                }
            } else {
                //random swap
                let d_1: i64 = rng.gen_range(1, lim_d + 1);
                let d_2: i64 = rng.gen_range(d_1, min(d_1 + 7, lim_d + 1));
                let con_1 = self.selected[d_1 as usize - 1];
                let con_2 = self.selected[d_2 as usize - 1];

                let new_score = self.change_and_rescore(d_1, con_2);
                let new_score = self.change_and_rescore(d_2, con_1);
                if new_score < old_score
                    && (!rng.gen_bool(f64::exp((new_score - old_score) as f64 / T)))
                {
                    self.change_and_rescore(d_1, con_1);
                    self.change_and_rescore(d_2, con_2);
                } else {
                    // eprintln!(
                    //     "s: i={}, {}->{} ({}), T={}",
                    //     i,
                    //     old_score,
                    //     new_score,
                    //     new_score - old_score,
                    //     T
                    // );
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    #[test]
    fn score_test() {
        let inp = "5
        86 90 69 51 2 96 71 47 88 34 45 46 89 34 31 38 97 84 41 80 14 4 50 83 7 82
        19771 12979 18912 10432 10544 12928 13403 3047 10527 9740 8100 92 2856 14730 1396 15905 6534 4650 11469 3628 8433 2994 10899 16396 18355 11424
        6674 17707 13855 16407 12232 2886 11908 1705 5000 1537 10440 10711 4917 10770 17272 15364 19277 18094 3929 3705 7169 6159 18683 15410 9092 4570
        6878 4239 19925 1799 375 9563 3445 5658 19857 11401 6997 6498 19933 3848 2426 2146 19745 16880 17773 18359 3921 14172 16730 11157 5439 256
        8633 15862 15303 10749 18499 7792 10317 5901 9395 11433 3514 3959 5202 19850 19469 9790 5653 784 18500 10552 17975 16615 7852 197 8471 7452
        19855 17918 7990 10572 4333 438 9140 9104 12622 4985 12319 4028 19922 12132 16259 17476 2976 547 19195 19830 16285 4806 4471 9457 2864 2192";

        input_text!(inp =>
            (d: i64)
            (c: [i64])
            {d; s: [i64]}
        );

        eprintln!("{:?}", d);
        eprintln!("{:?}", c);
        eprintln!("{:?}", s);
        {
            let mut DD = D.lock().unwrap();
            *DD = d;
            let mut CC = C.lock().unwrap();
            *CC = c;
            let mut SS = S.lock().unwrap();
            *SS = s;
        }

        let mut s = State::new(5);

        for (i_d, a) in enumerate([1, 17, 13, 14, 13]) {
            s.selected.push(a - 1);
            s.select_day[a - 1].push(i_d + 1);
        }

        assert_eq!(s.calc_score(), 79325);

        let incl_s = s.change_and_rescore(1, 1);
        // eprintln!("{:?}", s);
        assert_eq!(incl_s, 72553);
        assert_eq!(s.calc_score(), 72553);
    }
}

// ABC081A
// #[fastout]
fn main() {
    {
        *TIMER.lock().unwrap() = Some(Timer::new());
    }

    //new type
    let mut res = 0;

    input! {
        d: i64,
        c:[i64; 26],
        s: [[i64; 26]; d],
    };

    eprintln!("{:?}", d);
    eprintln!("{:?}", c);
    eprintln!("{:?}", s);
    eprintln!("{}", res);

    {
        let mut DD = D.lock().unwrap();
        *DD = d;
        let mut CC = C.lock().unwrap();
        *CC = c;
        let mut SS = S.lock().unwrap();
        *SS = s;
    }

    // let mut test_state = State::new(d as i32);
    // for (i_d, a) in enumerate([1, 17, 13, 14, 13]) {
    //     test_state.selected.push(a - 1);
    //     test_state.select_day[a - 1].push(i_d + 1);
    // }

    let mut best_state: Option<State> = None;
    for cost_interval in 0..100 {
        let mut init_state = State::new(d as i32);
        init_state.make_greedy(cost_interval);
        let new_score = init_state.calc_score();

        if let Some(best_state_in) = &best_state {
            if best_state_in.get_score() <= new_score {
                best_state = Some(init_state);
            }
        } else {
            best_state = Some(init_state);
        }
    }
    let mut best_state = best_state.unwrap();
    eprintln!("state:\n{:?}", best_state);
    let score = best_state.calc_score();
    // assert_eq!(score, 79325);
    eprintln!("score_before: {}", score);
    best_state.annealing_update();
    eprintln!("score_after: {}", best_state.calc_score());

    best_state.print_out();
}

// #[cfg(test)]
// mod tests{
//     use super::*;

//     [test]

// }
