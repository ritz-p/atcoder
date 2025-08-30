use proconio::input;
fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let mut v = vec![x, y];
    let mut index = 1;
    while index < 10 {
        let mut current = v[index] + v[index - 1];
        let l = current.to_string().len();
        let mut next = 0;
        for i in 0..l {
            let d = current / 10_usize.pow(l as u32 - i as u32 - 1);
            current %= 10_usize.pow(l as u32 - i as u32 - 1);
            next += d * 10_usize.pow(i as u32);
        }
        v.push(next);
        index += 1;
    }
    println!("{}", v[9]);
}
