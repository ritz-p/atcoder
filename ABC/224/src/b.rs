use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w];h]
    };

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            if a[i][j] + a[i + 1][j + 1] > a[i + 1][j] + a[i][j + 1] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
