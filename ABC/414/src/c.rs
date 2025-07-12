use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    };
    let len = n.to_string().len();
    let mut res = 0;

    for l in 1..=len {
        let odd = l % 2 == 1;
        let half = (l + 1) / 2;
        let start = 10usize.pow((half - 1) as u32);
        let end = 10usize.pow(half as u32) - 1;
        for h in start..=end {
            let p = palidrome(h, odd);
            if p > n {
                if l == len {
                    break;
                } else {
                    continue;
                }
            }
            if base(p, a) {
                res += p;
            }
        }
    }

    println!("{}", res);
}
fn base(mut x: usize, b: usize) -> bool {
    let mut dig = Vec::new();
    while x > 0 {
        dig.push(x % b);
        x /= b;
    }
    if dig.is_empty() {
        dig.push(0);
    }
    let n = dig.len();
    for i in 0..n / 2 {
        if dig[i] != dig[n - 1 - i] {
            return false;
        }
    }
    true
}
fn palidrome(mut h: usize, odd: bool) -> usize {
    let mut p = h;
    if odd {
        h /= 10;
    }
    while h > 0 {
        p = p * 10 + (h % 10);
        h /= 10;
    }
    p
}
