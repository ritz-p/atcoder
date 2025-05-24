use proconio::{input, marker::Chars};
fn main() {
    input! {
        mut s: Chars
    };
    let mut res = 0;
    let mut total = 0;

    while s.len() != 0 {
        let l = s.len() - 1;
        let f = s[l] as usize - 48;
        s[l] = '0';
        let d = (f + 10 - (total % 10)) % 10;
        total += d;
        s.pop();
        res += 1 + d;
    }

    println!("{}", res);
}
