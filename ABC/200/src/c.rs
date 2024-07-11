use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut v = vec![0; 200];
    let mut res: usize = 0;
    for i in &a {
        v[*i % 200] += 1;
    }
    for e in v {
        res += e * (e - 1) / 2;
    }
    println!("{}", res);
}
