use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    };
    let mut count = 0;
    for i in 0..n {
        if s[i] != t[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
