use proconio::input;
fn main() {
    input! {
        n: usize,
        pab: [(isize,isize,isize);n],
        q: usize,
        x: [isize;q]
    };

    let mut zeros = vec![0; n];
    for i in 0..n - 1 {
        for j in i..n {}
    }
    for e in x {
        let mut current = e;
        for (index, (p, a, b)) in pab.iter().enumerate() {
            if *p >= current {
                current += a;
            } else {
                current = 0.max(current - b);
            }
        }

        println!("{}", current);
    }
}
