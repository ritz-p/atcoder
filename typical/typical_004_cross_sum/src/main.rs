use proconio::input;
fn main() {
    input!{
        h: usize,
        w: usize,
        hw: [[usize;w];h]
    }
    let mut vertical = vec![0;h];
    let mut horizontal = vec![0;w];
    for i in 0..h{
        for j in 0..w{
            vertical[i] += hw[i][j];
            horizontal[j] += hw[i][j];
        }
    }

    
    for i in 0..h{
        for j in 0..w{
            print!("{} ",vertical[i]+horizontal[j]-hw[i][j]);
        }
        println!();
    }
}
