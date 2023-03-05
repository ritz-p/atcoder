use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut two_mul: Vec<u64> = vec![0; n + 1];
    {
        let mut x = 1;
        while x * x <= n {
            two_mul[x * x] += 1;
            let mut y = x + 1;
            while x * y <= n {
                two_mul[x * y] += 2;
                y += 1;
            }
            x += 1;
        }
    }
    let mut count = 0;
    for i in 1..n {
        count += two_mul[i] * two_mul[n - i];
    }
    println!("{}", count);
}
