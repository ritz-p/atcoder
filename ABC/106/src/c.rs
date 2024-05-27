use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    for i in 0..k.min(s.len()) {
        if s[i] != '1' {
            println!("{}", s[i]);
            return;
        }
    }
    println!("1");
}
