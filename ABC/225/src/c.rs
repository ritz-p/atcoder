use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize;m];n]
    };
    let i_base = b[0][0] / 7;
    let j_base = b[0][0] % 7;
    for i in i_base..i_base + n{
        for j in j_base..j_base + m{
            if j != j_base + m - 1 && b[i-i_base][j-j_base] % 7 == 0{
                println!("No");
                return;
            }
            if b[i-i_base][j-j_base] != 7 * i + j{
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
