use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: [Chars;8]
    };
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                for k in 0..8 {
                    if s[i][k] != '#' {
                        s[i][k] = '@';
                    }
                    if s[k][j] != '#' {
                        s[k][j] = '@';
                    }
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '.' {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
