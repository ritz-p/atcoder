use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut res = usize::MAX;
    let mut j = 1000000;
    for i in 0..=1000000 {
        while sum(i, j) >= n {
            res = res.min(sum(i, j));
            if j == 0 {
                break;
            }
            j = j.saturating_sub(1);
        }
    }

    println!("{}", res);
}

fn sum(a: usize, b: usize) -> usize {
    return a.pow(3) + a.pow(2) * b + a * b.pow(2) + b.pow(3);
}
