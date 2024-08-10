use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[[usize;n];n];n],
        q: usize,
    };

    let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];

    for x in 1..=n {
        for y in 1..=n {
            for z in 1..=n {
                s[x][y][z] =
                    a[x - 1][y - 1][z - 1] + s[x - 1][y][z] + s[x][y - 1][z] + s[x][y][z - 1]
                        - s[x - 1][y - 1][z]
                        - s[x - 1][y][z - 1]
                        - s[x][y - 1][z - 1]
                        + s[x - 1][y - 1][z - 1];
            }
        }
    }
    for _i in 0..q {
        input! {
            lx: usize,
            rx: usize,
            ly: usize,
            ry: usize,
            lz: usize,
            rz: usize,
        };
        let res =
            s[rx][ry][rz] + s[lx - 1][ly - 1][rz] + s[lx - 1][ry][lz - 1] + s[rx][ly - 1][lz - 1]
                - s[lx - 1][ly - 1][lz - 1]
                - s[lx - 1][ry][rz]
                - s[rx][ly - 1][rz]
                - s[rx][ry][lz - 1];

        println!("{}", res);
    }
}
