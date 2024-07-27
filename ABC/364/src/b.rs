use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut si: usize,
        mut sj: usize,
        cm: [Chars;h],
        x: Chars
    };
    si -= 1;
    sj -= 1;
    for c in x {
        match c {
            'L' => {
                if sj > 0 {
                    if cm[si][sj - 1] == '.' {
                        sj -= 1;
                    }
                }
            }
            'R' => {
                if sj < w - 1 {
                    if cm[si][sj + 1] == '.' {
                        sj += 1;
                    }
                }
            }
            'U' => {
                if si > 0 {
                    if cm[si - 1][sj] == '.' {
                        si -= 1;
                    }
                }
            }
            'D' => {
                if si < h - 1 {
                    if cm[si + 1][sj] == '.' {
                        si += 1;
                    }
                }
            }
            _ => {}
        }
    }
    println!("{} {}", si + 1, sj + 1);
}
