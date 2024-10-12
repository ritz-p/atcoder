use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if n < 2 {
        println!("0");
        return;
    }
    let mut res = 0;
    for i in 0..n - 2 {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            res += 1;
        }
    }
    println!("{}", res);
}
