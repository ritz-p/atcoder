use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        q: usize,
    };
    for i in 1..n {
        a[i] += a[i - 1];
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };
        let current = a[r - 1] as isize - if l - 1 == 0 { 0 } else { a[l - 2] as isize };
        let lose = r as isize - l as isize + 1;
        if current * 2 > lose {
            println!("win");
        } else if current * 2 < lose {
            println!("lose");
        } else if current * 2 == lose {
            println!("draw");
        }
    }
}
