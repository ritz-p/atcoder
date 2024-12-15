use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut a: [usize;n]
    };
    for i in 0..n {
        while a[i] >= x && k > 0 {
            a[i] -= x;
            k -= 1;
        }
    }
    a.sort();
    a.reverse();
    for i in 0..k.min(n) {
        a[i] = 0;
    }

    let res = a.iter().sum::<usize>();
    println!("{}", res);
}
