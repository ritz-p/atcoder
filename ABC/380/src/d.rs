use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [usize;q]
    };
    let mut res = vec![];
    let l = s.len();
    for q in k {
        let div = (q - 1) / l;
        let modulo = (q - 1) % l;
        let c = s[modulo];
        if div.count_ones() % 2 == 0 {
            res.push(c);
        } else {
            res.push(invert(c));
        }
    }

    println!("{}", res.iter().join(" "));
}

fn invert(c: char) -> char {
    if c.is_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}
