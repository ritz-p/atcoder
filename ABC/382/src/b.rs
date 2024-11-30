use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut d: usize,
        mut s: Chars
    };
    for i in (0..n).rev() {
        if s[i] == '@' && d > 0 {
            s[i] = '.';
            d -= 1;
        }
    }

    println!("{}", s.iter().collect::<String>());
}
