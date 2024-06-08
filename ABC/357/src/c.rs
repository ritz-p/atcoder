use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let l = 3_usize.pow(n as u32);
    let mut map = vec![vec!['#';l];l];
    gen(n,l,0,0,&mut map);
    for i in 0..l{
        for j in 0..l{
            print!("{}",map[i][j]);
        }
        println!();
    }
}

fn gen(n: usize, l: usize, x: usize, y: usize, map: &mut Vec<Vec<char>>) {
    if n == 0 {
        map[x][y] = '#';
        return;
    }
    let block_size = 3_usize.pow((n - 1) as u32);
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                for k in 0..block_size {
                    for l in 0..block_size {
                        map[x + i * block_size + k][y + j * block_size + l] = '.';
                    }
                }
            } else {
                gen(n - 1, block_size, x + i * block_size, y + j * block_size, map);
            }
        }
    }
}