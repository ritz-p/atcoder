use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    };
    for i in 0..n {
        for j in 0..n {
            let mut current = 0;
            if j + 5 <= n - 1 {
                for k in 0..6 {
                    if s[i][j + k] == '#' {
                        current += 1;
                    }
                }
            }
            if current >= 4 {
                println!("Yes");
                return;
            }
            current = 0;
            if i + 5 <= n - 1 {
                for k in 0..6 {
                    if s[i + k][j] == '#' {
                        current += 1;
                    }
                }
            }
            if current >= 4 {
                println!("Yes");
                return;
            }
            current = 0;
            if i + 5 <= n - 1 && j + 5 <= n - 1 {
                for k in 0..6 {
                    if s[i + k][j + k] == '#' {
                        current += 1;
                    }
                }
            }
            if current >= 4 {
                println!("Yes");
                return;
            }
            current = 0;
            if i + 5 <= n - 1 && j >= 5 {
                for k in 0..6 {
                    if s[i + k][j - k] == '#' {
                        current += 1;
                    }
                }
            }
            if current >= 4 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
