use proconio::input;

fn main() {
    input! {
        n: usize,
        cp_list: [(usize, usize); n],
        q: usize,
        lr_list: [(usize, usize); q],
    }

    let mut p1 = vec![0];
    let mut p2 = vec![0];

    for (c, p) in cp_list {
        match c {
            1 => {
                p1.push(p1.last().unwrap() + p);
                p2.push(*p2.last().unwrap());
            }
            2 => {
                p1.push(*p1.last().unwrap());
                p2.push(p2.last().unwrap() + p);
            }
            _ => unreachable!(),
        }
    }
// それぞれの段階での合計を求めてあとで引く
    for (l, r) in lr_list {
        let s1 = p1[r] - p1[l - 1];
        let s2 = p2[r] - p2[l - 1];
        println!("{s1} {s2}");
    }
}
