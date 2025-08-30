use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    };
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                x[i] += 1;
                y[j] += 1;
            }
        }
    }
    let mut res: usize = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                res += (x[i] - 1) * (y[j] - 1);
            }
        }
    }
    println!("{}", res);
}
