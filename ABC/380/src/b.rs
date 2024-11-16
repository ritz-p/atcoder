use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let mut v = vec![];

    let mut current = 0;
    for i in 1..s.len() {
        match s[i] {
            '-' => {
                current += 1;
            }
            '|' => {
                v.push(current);
                current = 0;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", v.iter().join(" "));
}
