use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    let res = (s[0] as usize - 48) * (s[2] as usize - 48);

    println!("{}", res);
}
