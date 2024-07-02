use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    };
    if (s[0] == 'M' && s[1] == 'R') || (s[0] == 'M' && s[2] == 'R') || (s[1] == 'M' && s[2] == 'R')
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
