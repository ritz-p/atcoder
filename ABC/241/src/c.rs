use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    };
    let mut flag = false;
    for i in 0..n {
        for j in 0..n {
            if i < n - 5 {
                let mut count = 0;
                for k in 0..6 {
                    if s[i + k][j] == '#' {
                        count += 1;
                    }
                    if count >= 4 {
                        flag = true;
                    }
                }
            }
            if j < n - 5 {
                let mut count = 0;
                for k in 0..6 {
                    if s[i][j + k] == '#' {
                        count += 1;
                    }
                    if count >= 4 {
                        flag = true;
                    }
                }
            }
            if i < n - 5 && j < n - 5 {
                let mut count = 0;
                for k in 0..6 {
                    if s[i + k][j + k] == '#' {
                        count += 1;
                    }
                    if count >= 4 {
                        flag = true;
                    }
                }
            }
            if i >= 5 && j < n - 5 {
                let mut count = 0;
                for k in 0..6 {
                    if s[i - k][j + k] == '#' {
                        count += 1;
                    }
                    if count >= 4 {
                        flag = true;
                    }
                }
            }
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
