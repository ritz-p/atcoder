use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        lr: [(isize,isize);n]
    };
    let mut res: Vec<isize> = vec![0; n];
    let mut sum = 0;
    let mut x = 0;

    for (i, &(l, r)) in lr.iter().enumerate() {
        res[i] = l;
        sum += l;
        x += r;
    }

    if sum > 0 || x < 0 {
        println!("No");
        return;
    }

    for (i, &(l, r)) in lr.iter().enumerate() {
        if sum + (r - l) > 0 {
            res[i] = l + sum.abs_diff(0) as isize;
            break;
        } else {
            res[i] = r;
            sum += r - l;
        }
    }
    println!("Yes");
    println!("{}", res.iter().join(" "));
}
