use std::collections::HashSet;

use proconio::{input, marker::Chars};
fn main() {
    input! {
        ha: usize,
        _wa: usize,
        a: [Chars;ha],
        hb: usize,
        _hw: usize,
        b: [Chars;hb],
        hx: usize,
        _wx: usize,
        x: [Chars;hx]
    };
    let aset = a
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<(isize, isize)>>();
    let bset = b
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<(isize, isize)>>();
    let xset = x
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<(isize, isize)>>();

    for &(xx, xy) in &xset {
        for &(ax, ay) in &aset {
            let cx = xx - ax;
            let cy = xy - ay;
            let mut t = HashSet::new();
            t.extend(aset.iter().map(|&(dx, dy)| (dx + cx, dy + cy)));
            if !t.is_subset(&xset) {
                continue;
            }

            for &(xx, xy) in &xset {
                for &(bx, by) in &bset {
                    let cx = xx - bx;
                    let cy = xy - by;
                    let mut t = t.clone();
                    t.extend(bset.iter().map(|&(dx, dy)| (dx + cx, dy + cy)));
                    if t == xset {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
