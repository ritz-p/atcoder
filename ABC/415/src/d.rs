use proconio::input;
fn main() {
    input! {
        mut n: usize,
        m: usize,
        ab: [(usize,usize);m],
    };
    let mut v = vec![];

    for (a, b) in ab {
        v.push((a - b, a));
    }
    v.sort();
    let mut res = 0;

    for (diff, a) in v {
        if a > n {
            continue;
        }
        let current = (n - a) / diff + 1;
        res += current;
        n -= current * diff;
    }

    println!("{}", res);
}
