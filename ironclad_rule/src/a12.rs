use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    };

    let mut left = 1;
    let mut right = 1000000000;
    while left < right {
        let mid = (left + right) / 2;
        let mut f = false;
        let mut sum = 0;
        for e in &a {
            sum += mid / e;
        }
        if sum >= k {
            f = true;
        }

        if f {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", left);
}
