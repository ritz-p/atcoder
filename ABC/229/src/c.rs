use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize,usize);n]
    };
    let mut res = 0;
    ab.sort_by(|a, b| b.0.cmp(&a.0));

    for (a, b) in ab {
        if b <= w {
            res += a * b;
            w -= b;
        } else {
            res += a * w;
            break;
        }
    }

    println!("{}", res);
}
