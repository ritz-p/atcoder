use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u128;n]
    };
    let mut res = 1;
    let p = 10_u128.pow(k as u32);
    for e in a {
        if res * e >= p {
            res = 1;
        } else {
            res *= e;
        }
    }

    println!("{}", res);
}
