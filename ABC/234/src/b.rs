use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64,f64);n]
    };
    let mut res: f64 = 0.0;

    for (i, (x1, y1)) in xy.iter().enumerate() {
        for (j, (x2, y2)) in xy.iter().enumerate() {
            if i == j {
                continue;
            }
            let distance = ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
            res = res.max(distance);
        }
    }
    println!("{}", res);
}
