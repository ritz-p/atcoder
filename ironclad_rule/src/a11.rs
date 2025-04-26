use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n]
    };

    let mut left = 0;
    let mut right = n;

    while left <= right {
        let mid = (left + right) / 2;
        if a[mid] > x {
            right = mid - 1;
        } else if a[mid] < x {
            left = mid + 1;
        } else {
            println!("{}", mid + 1);
            return;
        }
    }
}
