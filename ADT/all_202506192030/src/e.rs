use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    };
    a.sort();
    let mut count = 0;

    let mut right = n;
    for left in 0..n {
        right = right.max(left + 1);
        while right - 1 > left && a[right - 1] + a[left] >= 100000000 {
            right -= 1;
        }
        count += n - right;
    }

    let mut res = 0;

    for i in 0..n {
        res += a[i] * (n - 1);
    }
    res -= 100000000 * count;

    println!("{}", res);
}
