use proconio::input;
fn main() {
    input! {
        n: usize,
        k: u32,
        a: [usize;n]
    };

    let mut res: usize = 1;

    for e in a {
        res = res.saturating_mul(e);
        if res > 10_usize.pow(k) - 1 {
            res = 1;
        }
    }

    println!("{}", res);
}
