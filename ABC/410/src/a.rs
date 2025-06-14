use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        k: usize,
    };
    let mut res = 0;

    for e in a {
        if e >= k {
            res += 1;
        }
    }

    println!("{}", res);
}
