use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        mut s: [Chars;n],
        t: [Chars;n]
    };
    let mut res = usize::MAX;

    for i in 0..4 {
        let mut count = i;

        for i in 0..n {
            for j in 0..n {
                if s[i][j] != t[i][j] {
                    count += 1;
                }
            }
        }
        res = res.min(count);
        s = rotate(s, n);
    }

    println!("{}", res);
}

fn rotate(s: Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            res[i].push(s[n - 1 - j][i]);
        }
    }
    res
}
