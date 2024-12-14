use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: isize,
        da: [(usize,isize);n]
    };
    for (d, a) in da {
        match d {
            1 => {
                if r >= 1600 && r <= 2799 {
                    r += a;
                }
            }
            2 => {
                if r >= 1200 && r <= 2399 {
                    r += a;
                }
            }
            _ => {}
        }
    }

    println!("{}", r);
}
