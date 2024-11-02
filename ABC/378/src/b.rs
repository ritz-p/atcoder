use proconio::input;

fn main() {
    input! {
        n: usize,
        qr: [(usize,usize);n],
        q: usize,
        td: [(usize,usize);q]
    };

    for (t, d) in &td {
        let (q, r) = qr[t - 1];
        let rem = d % q;
        let de = (r + q - rem) % q;
        let next = d + de;
        println!("{}", next);
    }
}
