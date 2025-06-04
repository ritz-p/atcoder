use proconio::{input, marker::Chars};
fn main() {
    input! {
        t: usize,
    };
    let mut ns = vec![];
    for _i in 0..t {
        input! {
            n: usize,
            s: Chars
        };
        ns.push((n, s));
    }

    for (n, s) in ns {
        let mut c = vec![0_isize; n + 1];
        for i in 0..n {
            if s[i] == '0' {
                c[i + 1] = c[i] + 1;
            } else {
                c[i + 1] = c[i] - 1;
            }
        }
        let sum = s.iter().filter(|e| **e == '1').count() as isize;
        let mut res = 0;
        let mut current = 0;
        for i in 0..=n {
            res = res.min(c[i] - current);
            current = current.max(c[i]);
        }
        println!("{}", sum + res);
    }
}
