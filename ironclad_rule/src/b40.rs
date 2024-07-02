use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut count = vec![0; 100];
    for e in a {
        count[e % 100] += 1;
    }

    let mut res = 0;
    for i in 1..50 {
        res += count[i] * count[100 - i];
    }
    res += count[0] * (count[0] - 1) / 2;
    res += count[50] * (count[50] - 1) / 2;

    println!("{}", res);
}
