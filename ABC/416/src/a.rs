use proconio::{input, marker::Chars};
fn main() {
    input! {
        _n: usize,
        l: usize,
        r: usize,
        s: Chars
    };

    for i in l - 1..r {
        if s[i] != 'o' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
