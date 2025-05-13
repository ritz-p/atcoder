use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    if s[s.len() - 1] == 'r' && s[s.len() - 2] == 'e' {
        println!("er");
    } else {
        println!("ist");
    }
}
