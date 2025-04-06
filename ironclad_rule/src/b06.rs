use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        q: usize,
        lr: [(usize,usize);q]
    };
    let mut sums = vec![0; n + 1];

    for i in 0..n {
        sums[i + 1] = a[i] + sums[i];
    }

    for (l, r) in &lr {
        if sums[*r] - sums[l - 1] == (r - l + 1) / 2 && (r - l + 1) % 2 == 0 {
            println!("draw");
        } else if sums[*r] - sums[l - 1] > (r - l + 1) / 2 {
            println!("win");
        } else {
            println!("lose");
        }
    }
}
