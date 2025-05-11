use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let mut res = vec![];
    while n != 0 {
        if n % 2 == 0 {
            n /= 2;
            res.push('B');
        } else {
            n -= 1;
            res.push('A');
        }
    }
    println!("{}", res.iter().rev().collect::<String>());
}
