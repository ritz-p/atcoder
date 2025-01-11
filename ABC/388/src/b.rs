use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [(usize,usize);n]
    };
    for i in 1..=d {
        let mut m = 0;
        for (t, l) in &tl {
            m = m.max(t * (l + i));
        }
        println!("{}", m);
    }
}
