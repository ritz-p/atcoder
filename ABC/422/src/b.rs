use proconio::{input, marker::Chars};
fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars;h]
    };
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '#' {
                let mut count = 0;
                if i > 0 {
                    if ss[i - 1][j] == '#' {
                        count += 1;
                    }
                }
                if i < h - 1 {
                    if ss[i + 1][j] == '#' {
                        count += 1;
                    }
                }
                if j > 0 {
                    if ss[i][j - 1] == '#' {
                        count += 1
                    }
                }
                if j < w - 1 {
                    if ss[i][j + 1] == '#' {
                        count += 1;
                    }
                }
                if count != 2 && count != 4 {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
