use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        mut x: usize,
    };
    let sum: usize = a.iter().sum();
    let mut res = x / sum * n;
    x %= sum;
    for e in a {
        if x < e {
            res += 1;
            break;
        } else {
            x -= e;
            res += 1;
        }
    }
    println!("{}", res);
}
