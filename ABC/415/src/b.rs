use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    let f = s
        .iter()
        .enumerate()
        .map(|(i, &c)| (i, c))
        .filter(|(_, c)| *c == '#')
        .collect::<Vec<(usize, char)>>();

    for res in f.chunks(2) {
        println!("{},{}", res[0].0 + 1, res[1].0 + 1);
    }
}
