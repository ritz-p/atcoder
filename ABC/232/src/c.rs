use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m],
        cd: [(usize,usize);m],
    };
    let mut abg = vec![vec![false; n]; n];
    let mut cdg = vec![vec![false; n]; n];

    for (a, b) in &ab {
        abg[a - 1][b - 1] = true;
        abg[b - 1][a - 1] = true;
    }
    for (c, d) in &cd {
        cdg[c - 1][d - 1] = true;
        cdg[d - 1][c - 1] = true;
    }

    for perm in (0..n).permutations(n) {
        let mut f = true;
        for i in 0..n {
            for j in 0..n {
                if abg[i][j] != cdg[perm[i]][perm[j]] {
                    f = false;
                }
                if !f {
                    break;
                }
            }
            if !f {
                break;
            }
        }
        if f {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
