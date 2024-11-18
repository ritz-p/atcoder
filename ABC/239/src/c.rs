use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    for x in a - 2..a + 3 {
        for y in b - 2..b + 3 {
            if (a - x).pow(2) + (b - y).pow(2) == 5 && (c - x).pow(2) + (d - y).pow(2) == 5 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
