use proconio::input;
fn main() {
    input! {
        mut n: usize,
    };
    if n % 2 == 1 {
        println!("0");
        return;
    }
    let mut res = n / 10;
    n /= 10;
    let mut fives = 5;
    while fives <= n {
        res += n / fives;
        fives *= 5;
    }
    println!("{}", res);
}
