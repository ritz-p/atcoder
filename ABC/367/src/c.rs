use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize;n]
    }
    gen(n, &r, k, &mut Vec::new(), 0, 0);
}

fn gen(n: usize, r: &[usize], k: usize, current: &mut Vec<usize>, sum: usize, index: usize) {
    if index == n {
        if sum % k == 0 {
            println!("{}", current.iter().join(" "));
        }
        return;
    }
    for num in 1..=r[index] {
        current.push(num);
        gen(n, r, k, current, sum + num, index + 1);
        current.pop();
    }
}
