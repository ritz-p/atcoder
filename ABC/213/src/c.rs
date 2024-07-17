use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize,usize);n]
    };
    let mut v = vec![];
    let mut h = vec![];
    for (a, b) in &ab {
        v.push(*a);
        h.push(*b);
    }
    v.sort();
    h.sort();
    v.dedup();
    h.dedup();

    for (a, b) in &ab {
        let a_pos = v.binary_search(a).unwrap();
        let b_pos = h.binary_search(b).unwrap();
        println!("{} {}", a_pos + 1, b_pos + 1);
    }
}
