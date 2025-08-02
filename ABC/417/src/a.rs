use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars
    };
    for i in a..n - b {
        print!("{}", s[i]);
    }
    println!();
}
