use proconio::input;

fn main() {
    input! {
        a: isize, b: isize, c: isize,
        d: isize, e: isize, f: isize,
        g: isize, h: isize, i: isize,
        j: isize, k: isize, l: isize,
    };

    let x_cover = d > g && j > a;
    let y_cover = e > h && k > b;
    let z_cover = f > i && l > c;

    let cover = x_cover && y_cover && z_cover;

    if cover {
        println!("Yes");
    } else {
        println!("No");
    }
}
