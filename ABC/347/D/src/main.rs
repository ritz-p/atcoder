use proconio::input;
fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128
    }
    let n = c.count_ones() as i128;
    if (n + (a + b)) % 2 == 1 {
        println!("-1");
        return;
    }
    let x = (n + (a - b)) / 2; // x側だけ1にする個数
    let z = a - x; // 両方1にする個数

    if n > a + b || a > n + b || b > n + a || n + a + b > 120 {
        println!("-1");
        return;
    }

    let mut ans1: i128 = 0;
    let mut ans2: i128 = 0;
    let mut counter = 0;
    let mut counter2 = 0;
    for i in 0..60 {
        if c >> i & 1 == 1 {
            if counter < x {
                ans1 += 1 << i;
                counter += 1;
            } else {
                ans2 += 1 << i;
            }
        } else {
            if counter2 < z {
                ans1 += 1 << i;
                ans2 += 1 << i;
                counter2 += 1;
            }
        }
    }

    println!("{} {}", ans1, ans2);
}
