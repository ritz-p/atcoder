use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    let mut f = vec![true; 24];
    if b > c {
        for i in b..24 {
            f[i] = false;
        }
        for i in 0..c {
            f[i] = false;
        }
    } else {
        for i in b..c {
            f[i] = false;
        }
    }
    if f[a] {
        println!("Yes");
    } else {
        println!("No");
    }
}
