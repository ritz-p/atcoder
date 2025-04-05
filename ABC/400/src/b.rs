use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut res = 0;
    for i in 0..=m {
        res += n.pow(i as u32);
        if res > 1000000000 {
            println!("inf");
            return;
        }
    }
    println!("{}", res);
}
