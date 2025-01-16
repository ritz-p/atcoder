use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let n_sum = n * (n + 1) / 2;
    let a_sum = n / a * (n / a + 1) / 2 * a;
    let b_sum = n / b * (n / b + 1) / 2 * b;
    let lcm = lcm(a, b);
    let ab_sum = n / lcm * (n / lcm + 1) / 2 * lcm;

    println!("{}", n_sum - a_sum - b_sum + ab_sum);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
