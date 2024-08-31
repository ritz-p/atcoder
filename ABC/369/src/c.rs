use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize;n]
    };
    let mut res: usize = 0;
    let mut count: usize = 1;

    for i in 1..n {
        if i == 1 || a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
            count += 1;
        } else {
            res += count * (count - 1) / 2;
            count = 2;
        }
    }
    res += count * (count - 1) / 2;

    println!("{}", res + n);
}
