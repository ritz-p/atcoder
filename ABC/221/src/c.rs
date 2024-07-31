use proconio::{marker::Chars, *};

fn main() {
    input! { n: Chars }
    let mut res = 0;
    for i in 0..1 << n.len() {
        let mut a = vec![];
        let mut b = vec![];
        for j in 0..n.len() {
            if i >> j & 1 == 0 {
                a.push(n[j] as usize - b'0' as usize);
            } else {
                b.push(n[j] as usize - b'0' as usize);
            }
        }
        a.sort();
        b.sort();
        a.reverse();
        b.reverse();
        let mut x = 0;
        for a in a {
            x = x * 10 + a;
        }
        let mut y = 0;
        for b in b {
            y = y * 10 + b;
        }
        res = res.max(x * y);
    }
    println!("{}", res);
}
