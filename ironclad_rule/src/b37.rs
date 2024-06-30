use proconio::input;

fn main() {
    input! {
        n: String,
    };
    let mut res = 0;
    let mut r = vec![vec![0; 10]; 18];

    for i in 0..=14 {
        let nu = n.parse::<usize>().unwrap();
        let dig = nu / 10_usize.pow(i) % 10;

        for j in 0..10 {
            if j < dig {
                r[i as usize][j] = nu / 10_usize.pow(i + 1) * 10_usize.pow(i) + 10_usize.pow(i);
            } else if j == dig {
                r[i as usize][j] =
                    nu / 10_usize.pow(i + 1) * 10_usize.pow(i) + nu % 10_usize.pow(i) + 1;
            } else {
                r[i as usize][j] = nu / 10_usize.pow(i + 1) * 10_usize.pow(i);
            }
        }
    }
    for i in 0..=15 {
        for j in 0..10 {
            res += 1 * j * r[i][j];
        }
    }

    println!("{}", res);
}
