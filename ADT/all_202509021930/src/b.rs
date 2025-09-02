use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars
    };
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
