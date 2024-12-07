use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(usize,usize);n]
    };
    let mut res = tv[0].1;
    for i in 1..n {
        let diff = tv[i].0 - tv[i - 1].0;
        if diff > res {
            res = 0;
        } else {
            res -= diff;
        }
        res += tv[i].1;
    }
    println!("{}", res);
}
