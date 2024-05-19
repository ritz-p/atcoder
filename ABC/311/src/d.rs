use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        n: usize,
        m: usize,
        arr: [Chars;n]
    };
    let mut flag = vec![vec![false;m];n];
    let mut visited = vec![(1,1)];

    while !visited.is_empty(){
        let t = visited.pop().unwrap();
        for &(i,j) in [(0,1),(1,0),(0,-1),(-1,0)].iter(){
            let (mut x,mut y) = t;
            while arr[(x+i) as usize][(y+j) as usize] != '#'{
                flag[x as usize][y as usize] = true;
                x += i;
                y += j;
            }

            if !flag[x as usize][y as usize]{
                visited.push((x,y));
                flag[x as usize][y as usize] = true;
            }
        }
    }
    let mut res = 0;
    for i in 0..n{
        for j in 0..m{
            res += flag[i][j] as usize;
        }
    }

    println!("{}",res);
}
