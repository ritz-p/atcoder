use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    if n % 2 != 1 {
        println!("No");
        return;
    }
    for i in 0..n / 2 {
        if s[i] != '1' {
            println!("No");
            return;
        }
    }
    if s[n / 2] != '/' {
        println!("No");
        return;
    }
    for i in n / 2 + 1..n {
        if s[i] != '2' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
