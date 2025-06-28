use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n*2]
    };
    let mut res = 0;

    for i in 0..n * 2 - 2 {
        if a[i] == a[i + 2] {
            res += 1;
        }
    }

    println!("{}", res);
}
