use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    println!("{}", (s[0] as u32 - 48) * (s[2] as u32 - 48))
}
