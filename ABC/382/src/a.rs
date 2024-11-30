use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        d: usize,
        s: Chars
    };
    let mut res = 0;
    for c in s {
        if c == '.' {
            res += 1;
        }
    }
    res += d;
    println!("{}", res);
}
