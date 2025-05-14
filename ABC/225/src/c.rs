use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize;m];n]
    };
    let ib = b[0][0] / 7;
    let jb = b[0][0] % 7;
    for i in ib..ib + n {
        for j in jb..jb + m {
            if j != jb + m - 1 && b[i - ib][j - jb] % 7 == 0 {
                println!("No");
                return;
            }
            if b[i - ib][j - jb] != 7 * i + j {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
