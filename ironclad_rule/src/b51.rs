use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    };
    let mut v = vec![];
    for (i, c) in s.iter().enumerate() {
        match c {
            '(' => {
                v.push(i + 1);
            }
            ')' => {
                println!("{} {}", v.pop().unwrap(), i + 1)
            }
            _ => {}
        }
    }
}
