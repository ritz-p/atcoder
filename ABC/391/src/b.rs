use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars;n],
        t: [Chars;m]
    };
    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut flag = true;
            for k in 0..m {
                for l in 0..m {
                    if s[i + k][j + l] != t[k][l] {
                        flag = false;
                        break;
                    }
                }
                if !flag {
                    break;
                }
            }
            if flag {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
