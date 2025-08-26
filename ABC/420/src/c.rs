use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        b: [usize;n],
        cxv: [(char,usize,usize);q]
    };
    let mut sum = 0;
    let mut mins = vec![];
    for (ae, be) in a.iter().zip(b.iter()) {
        sum += ae.min(be);
        mins.push(*ae.min(be));
    }

    let mut ac = a.clone();
    let mut bc = b.clone();

    for (c, x, v) in cxv {
        match c {
            'A' => {
                let current = mins[x - 1];
                ac[x - 1] = v;
                let min = ac[x - 1].min(bc[x - 1]);
                mins[x - 1] = min;
                sum += min;
                sum -= current;
                println!("{}", sum);
            }
            'B' => {
                let current = mins[x - 1];
                bc[x - 1] = v;
                let min = ac[x - 1].min(bc[x - 1]);
                mins[x - 1] = min;
                sum += min;
                sum -= current;
                println!("{}", sum);
            }
            _ => {}
        }
    }
}
