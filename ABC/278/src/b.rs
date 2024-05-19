use proconio::input;

fn main() {
    input! {
        h: i64,
        m: i64
    }
    let max = 24 * 60;
    for i in 0..max {
        let mi = m + i;
        let hh = (h + (mi / 60)) % 24;
        let mm = mi % 60;
        let h1 = hh / 10;
        let h2 = hh % 10;
        let m1 = mm / 10;
        let m2 = mm % 10;
        let hm1 = h1 * 10 + m1;
        let hm2 = h2 * 10 + m2;
        // print!("{}",i);
        if hm1 >= 0 && hm1 < 24 && hm2 >= 0 && hm2 < 60 {
            println!("{} {}", hh, mm);
            break;
        }
    }
}
