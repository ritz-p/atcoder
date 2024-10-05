use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let l = s.len().min(t.len());
    for i in 0..l {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
    if l < s.len() || l < t.len() {
        println!("{}", l + 1);
        return;
    }

    println!("0");
}
