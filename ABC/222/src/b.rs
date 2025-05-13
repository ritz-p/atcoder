use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize;n]
    };
    let res = a.iter().filter(|v| v < &&p).count();
    println!("{}", res);
}
