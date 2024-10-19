use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        mut b: [usize;n-1]
    };
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    if a[n - 1] > b[n - 2] {
        println!("-1");
        return;
    }
    let mut count = 0;
    let mut res = a[n - 1] as isize;
    for i in 0..n - 1 {
        if a[i] > b[i - count] {
            count += 1;
            res = a[i] as isize;
        }
        if count >= 2 {
            println!("-1");
            return;
        }
    }

    println!("{}", res);
}
