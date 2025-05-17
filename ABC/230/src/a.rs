use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    if n >= 42 {
        n += 1;
    }
    println!("AGC{}", format!("{:0>3}", n));
}
