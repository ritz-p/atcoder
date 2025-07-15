use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    if s[0] == 'R' {
        println!("Yes");
    } else if s[1] == 'R' {
        if s[2] == 'M' {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
