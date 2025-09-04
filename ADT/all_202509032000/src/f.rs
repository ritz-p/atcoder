use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars;n]
    };
    let mut res = usize::MAX;
    for b in 0..(1 << n) {
        let mut f = vec![false; m];
        let mut current = 0;
        for i in 0..n {
            if (b >> i) & 1 != 0 {
                current += 1;
                for j in 0..m {
                    if s[i][j] == 'o' {
                        f[j] = true;
                    }
                }
            }
        }
        if f.iter().all(|flag| *flag) {
            res = res.min(current);
        }
    }
    println!("{}", res);
}
