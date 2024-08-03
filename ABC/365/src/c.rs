use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n]
    };
    let mut left: usize = 0;
    let mut right: usize = 1000000001;

    while left + 1 < right {
        let mid = (left + right) / 2;
        let mut total = 0;
        for e in &a {
            total += e.min(&mid);
        }
        if total <= m {
            left = mid;
        } else {
            right = mid;
        }
    }
    if right == 1000000001 {
        println!("infinite");
    } else {
        println!("{}", left);
    }
}
