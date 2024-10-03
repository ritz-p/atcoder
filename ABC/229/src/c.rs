use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize,usize);n]
    };
    ab.sort_by(|a, b| b.0.cmp(&a.0));
    let mut res = 0;
    for (a, b) in ab {
        if b <= w {
            res += a * b;
            w -= b;
        } else {
            res += a * w;
            w = 0;
        }
        if w == 0 {
            break;
        }
    }
    println!("{}", res);
}
