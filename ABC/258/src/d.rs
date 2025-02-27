use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize);n]
    };
    let mut res = usize::MAX;
    let mut sum = 0;
    for (index, (a, b)) in ab.iter().enumerate() {
        if index + 1 > x {
            break;
        }
        sum += a + b;
        res = res.min(sum + b * (x - index - 1));
    }
    println!("{}", res);
}
