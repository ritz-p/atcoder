use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars
    };
    if n < 3 {
        println!("No");
        return;
    }
    if s[n - 1] == 'a' && s[n - 2] == 'e' && s[n - 3] == 't' {
        println!("Yes");
    } else {
        println!("No");
    }
}
