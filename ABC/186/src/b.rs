use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }
    let mut res:i32 = 0;
    let m = a.iter().flatten().min().unwrap();
    res += a.iter().flatten().map(|x| x - m).sum::<i32>();
    println!("{}",res);
}
