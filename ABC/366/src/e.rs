use proconio::input;

fn main() {
    input! {
        n: usize,
        d: isize,
        xy: [(isize,isize);n]
    };
    let mut xs: Vec<isize> = xy.iter().map(|&(x, _)| x).collect();
    let mut ys: Vec<isize> = xy.iter().map(|&(_, y)| y).collect();

    xs.sort();
    ys.sort();

    let mx = xs[n / 2];
    let my = ys[n / 2];

    let mut res = 0;
    for x in mx - d..=mx + d {
        for y in my - d..=my + d {
            let mut dis = 0;
            for &(xi, yi) in &xy {
                dis += (x - xi).abs() + (y - yi).abs();
            }
            if dis <= d {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
