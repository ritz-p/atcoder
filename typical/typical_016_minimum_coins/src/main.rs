use proconio::input;
fn main() {
    input!{
        mut n: i64,
        a: i64,
        b: i64,
        c: i64
    };

    let mut res:i64 = 10000;
    let max = 9999;
    for i in 0..=max{
        for j in 0..=max-i{
            let total = i * a + j * b;
            if n - total >= 0 && (n - total) % c == 0{
                res = res.min(i + j + (n-total) / c);
            }
        }
    }

    println!("{}",res);
}
