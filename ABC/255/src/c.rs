use proconio::input;

fn main() {
    input! {
        x: isize,
        mut a: isize,
        mut d: isize,
        n: isize,
    };
    if d < 0 {
        a = a + d * (n - 1);
        d *= -1;
    }
    if x <= a {
        println!("{}", a - x);
        return;
    }
    let max = a + d * (n - 1);
    if max <= x {
        let res = x - max;
        println!("{}", res);
        return;
    }

    let xad = (x - a) / d;
    let res = ((x - (a + d * xad)).abs()).min((x - (a + d * (xad + 1))).abs());
    println!("{}", res);
}
