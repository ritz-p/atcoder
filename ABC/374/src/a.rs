use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    if s[s.len() - 1] == 'n' && s[s.len() - 2] == 'a' && s[s.len() - 3] == 's' {
        println!("Yes");
    } else {
        println!("No");
    }
}