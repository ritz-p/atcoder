use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    };
    a.insert(0, 0);
    let mut pos = vec![0; n + 1];
    let mut res = 0;

    for i in 1..=n {
        let v = a[i];
        let p = pos[v];
        let sum = (i as usize - p as usize) * (n as usize - i as usize + 1);
        res += sum;
        pos[v] = i;
    }

    println!("{}", res);
}
