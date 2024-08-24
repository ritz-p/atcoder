use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    };
    let mut count = 0;
    loop {
        a.sort_by(|x, y| y.cmp(x));
        if a[1] <= 0 {
            break;
        }
        a[0] -= 1;
        a[1] -= 1;
        count += 1;
    }
    println!("{}", count);
}
