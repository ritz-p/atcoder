use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize);n]
    };
    let mut v = vec![];
    for &(a, b) in &ab {
        v.push((a, 1));
        v.push((a + b, -1));
    }
    v.sort_unstable();

    let mut res = vec![0; n + 1];
    let mut count = 0;

    for i in 0..v.len() - 1 {
        count += v[i].1;
        res[count as usize] += v[i + 1].0 - v[i].0;
    }

    for i in 1..=n {
        print!("{} ", res[i]);
    }
    println!();
}
