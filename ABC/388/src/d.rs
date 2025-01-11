use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut stones = vec![0; n + 1];
    let mut delta = vec![0; n + 2];
    for i in 1..=n {
        stones[i] = a[i - 1];
    }

    let mut give: isize = 0;

    for y in 1..=n {
        give += delta[y];
        stones[y] += give as usize;

        let sum = y + stones[y];
        if y + 1 <= n {
            delta[y + 1] += 1;
        }
        if sum + 1 <= n {
            delta[sum + 1] -= 1;
        }
    }
    for i in 1..=n {
        let count = std::cmp::min(stones[i], n - i);
        print!("{} ", stones[i] - count);
    }
    println!();
}
