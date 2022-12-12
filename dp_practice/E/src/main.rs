use proconio::input;

fn main() {
    input!{
        n: usize,
        max_w: usize,
        wv: [(usize,usize);n],
    };
    let mut prev = vec![max_w + 1; 100001];
    prev[0] = 0;
    for (w, v) in wv {
        let mut next = vec![max_w + 1; 100001];
        for (i, &p) in prev.iter().enumerate() {
            if p <= max_w{
                next[i] = next[i].min(prev[i]);
                if p + w <= max_w{
                    next[i+v] = (prev[i] + w).min(prev[i+v]);
                }
            }
        }
        prev = next;
    }
    println!("{}", prev.iter().rposition(|&e| e <= max_w).unwrap());
}
