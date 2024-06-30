use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize;n],
        c: [usize;m]
    };
    let res: usize = a.iter().map(|e| e * m).sum::<usize>()
        + c.iter().map(|e| e * n).sum::<usize>()
        + b * (n * m);

    println!("{}", res);
}
