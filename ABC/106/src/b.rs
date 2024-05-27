use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 0..=n {
        if i % 2 == 0 {
            continue;
        }
        let mut m = 0;
        for j in 1..=i {
            if i % j == 0 {
                m += 1;
            }
        }
        if m == 8 {
            res += 1;
        }
    }
    println!("{}",res);
}
