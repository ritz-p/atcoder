use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h]
    };
    let mut left = w - 1;
    let mut right = 0;
    let mut up = h - 1;
    let mut down = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                left = left.min(j);
                right = right.max(j);
                up = up.min(i);
                down = down.max(i);
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if j >= left && j <= right && i >= up && i <= down {
                if s[i][j] == '.' {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
