use proconio::input;

fn main() {
    input! {
        a: isize, b: isize, c: isize,
        d: isize, e: isize, f: isize,
        g: isize, h: isize, i: isize,
        j: isize, k: isize, l: isize,
    };

    let (x1_min, x1_max) = if a < d { (a, d) } else { (d, a) };
    let (y1_min, y1_max) = if b < e { (b, e) } else { (e, b) };
    let (z1_min, z1_max) = if c < f { (c, f) } else { (f, c) };

    let (x2_min, x2_max) = if g < j { (g, j) } else { (j, g) };
    let (y2_min, y2_max) = if h < k { (h, k) } else { (k, h) };
    let (z2_min, z2_max) = if i < l { (i, l) } else { (l, i) };

    let x_cover = x1_max > x2_min && x2_max > x1_min;
    let y_cover = y1_max > y2_min && y2_max > y1_min;
    let z_cover = z1_max > z2_min && z2_max > z1_min;

    let cover = x_cover && y_cover && z_cover;

    if cover {
        println!("Yes");
    } else {
        println!("No");
    }
}