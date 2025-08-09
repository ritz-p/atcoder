use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    let mut v = vec![];
    let l = s.len();
    for i in 0..l {
        if s[i] == 't' {
            v.push(i);
        }
    }
    let mut res = 0.0;
    let vl = v.len();

    for i in 0..vl {
        for j in i + 1..vl {
            let diff = v[j] - v[i];
            if diff >= 2 {
                let ts = (j - i - 1) as f64;
                let total = (diff - 1) as f64;
                let current = ts / total;
                if current > res {
                    res = current;
                }
            }
        }
    }
    println!("{}", res);
}
