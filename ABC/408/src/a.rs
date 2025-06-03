use proconio::input;
fn main() {
    input! {
        n: usize,
        s: usize,
        t: [usize;n]
    };
    let mut current = 0;
    for e in t {
        if e - current > s {
            println!("No");
            return;
        } else {
            current = e;
        }
    }
    println!("Yes");
}
