use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(usize,char);q]
    };
    let mut res = 0;
    for i in 0..n.saturating_sub(2) {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            res += 1;
        }
    }

    for (x, c) in xc {
        let x = x - 1;

        let start = x.saturating_sub(2);
        let end = (x + 1).min(n - 2);

        for i in start..=end {
            if s.get(i) == Some(&'A') && s.get(i + 1) == Some(&'B') && s.get(i + 2) == Some(&'C') {
                res -= 1;
            }
        }
        s[x] = c;
        for i in start..=end {
            if s.get(i) == Some(&'A') && s.get(i + 1) == Some(&'B') && s.get(i + 2) == Some(&'C') {
                res += 1;
            }
        }
        println!("{}", res);
    }
}
