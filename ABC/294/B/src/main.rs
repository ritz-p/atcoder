use proconio::input;

fn main(){
    input!{
        h: usize,
        w: usize,
        grid: [[usize;w];h],
    };
    let s = ".ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut arr = vec![];
    for i in 0..h{
        let mut t = "".to_string();
        for j in 0..w{
            t += &s.chars().nth(grid[i][j]).unwrap().to_string();
        }
        arr.push(t);
    }
    for i in 0..h{
        println!("{}",arr[i]);
    }
}