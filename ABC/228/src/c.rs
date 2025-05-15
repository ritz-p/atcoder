use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize;3];n]
    }
    let mut sums = vec![];
    for (index, v) in p.iter().enumerate() {
        let sum = v.iter().sum::<usize>();
        sums.push((sum, index));
    }
    sums.sort_by(|a, b| b.0.cmp(&a.0));
    let base = sums[k - 1].0;
    sums.sort_by(|a, b| a.1.cmp(&b.1));

    for (sum, _i) in sums {
        if sum + 300 >= base {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
