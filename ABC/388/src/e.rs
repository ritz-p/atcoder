use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut left = 0;
    let mut right = n / 2 + 1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        let mut flag = true;
        for i in 0..mid {
            if a[i] * 2 > a[n - mid + i] {
                flag = false;
                break;
            }
        }
        if flag {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
