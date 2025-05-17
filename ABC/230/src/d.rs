use proconio::input;

fn main() {
    input! {
        n: usize,
        d: isize,
        mut lr: [(isize,isize);n]
    };

    lr.sort_by(|a, b| a.1.cmp(&b.1));

    let mut res = 0;
    let mut current = isize::MIN;

    for (l, r) in lr {
        if l > current + d - 1 {
            current = r;
            res += 1;
        }
    }
    println!("{}", res);
}
