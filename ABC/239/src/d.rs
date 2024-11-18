use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let mut prime = vec![true; 201];
    prime[0] = false;
    prime[1] = false;

    for p in 0..15 {
        if prime[p] {
            let mut i = p * p;
            while i < 201 {
                prime[i] = false;
                i += p;
            }
        }
    }

    for i in a..=b {
        let mut f = false;
        for j in c..=d {
            if prime[i + j] {
                break;
            }
            if j == d {
                f = true;
            }
        }
        if f {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
