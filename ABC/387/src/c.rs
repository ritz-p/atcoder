use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    let lv = digit_vec(l - 1);
    let rv = digit_vec(r);

    println!("{}", count(rv) - count(lv));
}

fn digit_vec(n: usize) -> Vec<usize> {
    if n > 0 {
        let mut current = n;
        let mut v = vec![];
        while current != 0 {
            v.push(current % 10);
            current /= 10;
        }
        v.reverse();
        v
    } else {
        vec![0]
    }
}

fn count(digits: Vec<usize>) -> usize {
    let n = digits.len();
    let mut res = 0;
    for i in 1..=n {
        if i == n {
            res += 1;
            break;
        }
        res += digits[0].pow(n as u32 - i as u32 - 1) * digits[0].min(digits[i]);
        if digits[i] >= digits[0] {
            break;
        }
    }
    for i in 0..n {
        let mx = if i != 0 { 9 } else { digits[0] - 1 };
        for j in 1..=mx {
            res += j.pow(n as u32 - i as u32 - 1);
        }
    }
    res
}
