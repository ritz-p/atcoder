use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;
 
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    // vec! で初期化 w が要素数
    let mut counts = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] != '.' {
                counts[j] += 1;
            }
        }
    }
    // 要素の間にspace追加したものを代入
    let res = counts.iter().join(" ");
    println!("{}", res);
}