use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        t: [usize;n]
    };
    let mut res = 1;
    let mut current = t[0];
    for i in 1..n {
        if t[i] - current >= c {
            res += 1;
            current = t[i];
        }
    }

    println!("{}", res);
}
