use proconio::input;
fn main() {
    input! {
        n: usize,
        ab: [(usize,usize);n]
    };

    let mut res = 0;
    for (a, b) in ab {
        if b > a {
            res += 1;
        }
    }
    println!("{}", res);
}
