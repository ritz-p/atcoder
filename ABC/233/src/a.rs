use proconio::input;

fn main() {
    input! {
        mut x: usize,
        y: usize,
    };
    let mut res = 0;
    while y > x {
        res += 1;
        x += 10;
    }
    println!("{}", res);
}
