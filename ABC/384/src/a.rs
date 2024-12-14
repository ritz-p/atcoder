use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        c1: char,
        c2: char,
        s: Chars
    };
    for c in s {
        if c != c1 {
            print!("{}", c2);
        } else {
            print!("{}", c);
        }
    }
    println!();
}
