use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let m = 998244353;

    let mut res = 0;

    let s = n.to_string();

    let mut current = 1;
    for _i in 1..=s.len() {
        let diff = current * 10 - 1;
        let num = (n.min(diff) - current + 1) % m;
        let sum = (num + 1) * num / 2;
        res += sum;
        res %= m;
        current *= 10;
    }

    println!("{}", res);
}
