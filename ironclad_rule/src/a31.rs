use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let t = n / 3;
    let f = n / 5;
    let tf = n / 15;
    println!("{}", t + f - tf);
}
