use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut a: [usize;n]
    };
    let mut current = 0;
    let mut res = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i % 2 == 0 {
                res[i][j] = current + 1;
            } else {
                res[i][w - j - 1] = current + 1;
            }
            a[current] -= 1;
            if a[current] == 0 {
                current += 1;
            }
        }
    }
    for r in res {
        println!("{}", r.iter().join(" "));
    }
}
