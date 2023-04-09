use proconio::input;
const MAX: usize = std::usize::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = MAX;
    for a in 1..n + 1 {
        let b = (m - 1) / a + 1;
        if b <= n {
            ans = std::cmp::min(ans, a * b);
        }
        if b < a {
            break;
        }
    }
    if MAX == ans {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}