use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        xy: [(usize,usize);n]
    };
    let mut res = 0;
    for (x, y) in xy {
        if x <= l && y >= r {
            res += 1;
        }
    }

    println!("{}", res);
}
