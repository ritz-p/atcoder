use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize,usize,usize);n]
    };
    let mut days = vec![24; d];
    for (l, r, h) in lrh {
        for i in l - 1..r {
            days[i] = days[i].min(h);
        }
    }
    let res = days.iter().sum::<usize>();
    println!("{}", res);
}
