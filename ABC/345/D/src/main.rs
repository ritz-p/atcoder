use proconio::*;
use std::collections::HashSet;

fn main() {
    input!(n: usize, h: u32, w: u32,);

    let mut tiles = Vec::<Vec<i128>>::new();

    for _ in 0..n {
        input!(mut a: u32, mut b: u32,);

        let mut t = Vec::<i128>::new();
        for _ in 0..2 {
            if a <= h && b <= w {
                let mut tile = (1 << b) - 1;
                for _ in 0..(a - 1) {
                    tile |= tile << w;
                }
                t.push(tile);
            }
            if a == b {
                break;
            }
            std::mem::swap(&mut a, &mut b);
        }
        tiles.push(t);
    }

    let all_filled = (1 << (h * w)) - 1;
    let mut dp: HashSet<(i128, u32)> = HashSet::from([(0, 0)]);

    while !dp.is_empty() {
        let mut nx: HashSet<(i128, u32)> = HashSet::new();

        for (grid, used) in &dp {
            let x = grid ^ -1;
            let dist = (((x & -x) - 1) as u128).count_ones();
            let oh = dist / w;
            let ow = dist % w;
            for (i, t) in tiles.iter().enumerate().take(n) {
                if used >> i & 1 == 1 {
                    continue;
                }
                let id = 1 << i;
                for tile in t {
                    let x = tile ^ -1;
                    let tw = (((x & -x) - 1) as u128).count_ones().min(w);
                    let th = tile.count_ones() / tw;
                    if !(oh + th <= h && ow + tw <= w) || (grid & (tile << dist)) != 0 {
                        continue;
                    }
                    if (grid | (tile << dist)) == all_filled {
                        println!("Yes");
                        return;
                    }
                    nx.insert((grid | (tile << dist), used | id));
                }
            }
        }
        dp = nx;
    }
    println!("No")
}
