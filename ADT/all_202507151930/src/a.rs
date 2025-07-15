use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    let c = s.iter().filter(|v| **v == s[0]).count();

    if c == 1 {
        println!("1");
    } else {
        println!("{}", s.iter().position(|v| *v != s[0]).unwrap() + 1)
    }
}
