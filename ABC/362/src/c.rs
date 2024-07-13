use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(isize,isize);n]
    };
    let mut total_min = 0;
    let mut total_max = 0;
    let mut x = vec![0; n];

    for (index, (l, r)) in lr.iter().enumerate() {
        total_min += l;
        total_max += r;
        x[index] = *l;
    }
    if total_min > 0 || total_max < 0 {
        println!("No");
        return;
    }
    let mut sum = total_min;

    let mut index = 0;
    while sum < 0 {
        let (l, r) = lr[index];
        let d = r - l;
        let p = d.min(-sum);
        x[index] += p;
        sum += p;
        index += 1;
    }

    println!("Yes");
    println!("{}", x.iter().join(" "));
}
