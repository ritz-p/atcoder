use proconio::input;

fn main() {
    input! {
        d: i64,
    }
    let mut ans = 2000000000000i64;

    let d_sqrt = (d as f64).sqrt() as i64;

    for x in (1..=d_sqrt + 1).rev() {
        let remain = d - x * x;
        if remain < 0 {
            ans = ans.min((remain - 1 * 1).abs());
            continue;
        }
        let y = (remain as f64).sqrt() as i64;
        ans = ans.min((remain - y * y).abs());
        ans = ans.min((remain - (y + 1) * (y + 1)).abs());
    }
    println!("{}", ans);
}
