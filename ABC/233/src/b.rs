use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars
    };
    let cs = &s[l - 1..r];

    for i in 0..l - 1 {
        print!("{}", s[i]);
    }
    print!("{}", cs.iter().rev().collect::<String>());
    for i in r..s.len() {
        print!("{}", s[i]);
    }
    println!();
}
