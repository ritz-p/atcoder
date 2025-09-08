use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    if s[2] != '8' {
        let c = s[2] as u32 - 47;
        println!("{}-{}", s[0], c);
    } else {
        let c = s[0] as u32 - 47;
        println!("{}-1", c,);
    }
}
