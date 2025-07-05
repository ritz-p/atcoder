use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n]
    };
    let sum = a.iter().sum::<usize>();
    if m >= sum {
        println!("Yes");
    } else {
        println!("No");
    }
}
