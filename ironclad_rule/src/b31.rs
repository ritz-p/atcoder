use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let t = n / 3;
    let f = n / 5;
    let s = n / 7;
    let tf = n / (3 * 5);
    let ts = n / (3 * 7);
    let fs = n / (5 * 7);
    let tfs = n / (3 * 5 * 7);
    let res = t + f + s - tf - ts - fs + tfs;
    println!("{}", res);
}
