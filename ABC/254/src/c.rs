use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [isize; n],
    }

    let mut v = vec![vec![]; k];

    for (index, e) in a.iter().enumerate() {
        v[index % k].push(*e);
    }

    v.iter_mut().for_each(|v| v.sort_unstable());

    let b = (0..n).map(|i| v[i % k][i / k]).collect::<Vec<_>>();

    a.sort_unstable();
    if b == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
