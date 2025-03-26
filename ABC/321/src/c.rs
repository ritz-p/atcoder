#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        mut k:usize,
    }

    let mut v = vec![];
    for bit in 0..1 << 10 {
        let mut x = 0usize;
        for d in (0..=9).rev() {
            if (bit >> d) & 1 != 0 {
                x *= 10;
                x += d;
            }
        }

        if x != 0 {
            v.push(x);
        }
    }

    v.sort();

    let res = v[k - 1];
    println!("{}", res);
}
