use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        points: [usize;n],
        total: [usize;m],
    }
    let mut res = 0;
    for i in total{
        res += points[i-1];
    }
    println!("{}",res);
}