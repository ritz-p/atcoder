use proconio::{input, marker::Chars};
fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    };

    let mut res = vec![];
    for i in 0..s.len() {
        if s[i] != t[i] && s[i] > t[i] {
            s[i] = t[i];
            res.push(s.iter().collect::<String>());
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] != t[i] {
            s[i] = t[i];
            res.push(s.iter().collect::<String>());
        }
    }
    println!("{}", res.len());
    for x in res {
        println!("{}", x);
    }
}
