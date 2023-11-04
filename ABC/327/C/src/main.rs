use proconio::input;

fn main(){
    input!{
        grid: [[usize;9];9]
    };
    let pos = vec![(1,1),(4,4),(7,7),(4,1),(4,7),(1,4),(1,7),(7,1),(7,4)];
    for i in 0..9{
        let mut horizontal = vec![];
        let mut small = vec![];
        for j in 0..9{
            if !grid[i].contains(&(&j+1)){
                println!("No");
                return;
            }
            if pos.contains(&(i,j)){
                small.push(grid[i-1][j-1]);
                small.push(grid[i-1][j]);
                small.push(grid[i-1][j+1]);
                small.push(grid[i][j]);
                small.push(grid[i+1][j-1]);
                small.push(grid[i+1][j]);
                small.push(grid[i+1][j+1]);
                small.push(grid[i][j+1]);
                small.push(grid[i][j-1]);
                for k in 0..9{
                    if !small.contains(&(&k+1)){
                        println!("No");
                        return;
                    }
                }
                small = vec![];
            }
            horizontal.push(grid[j][i]);
        }
        for j in 0..9{
            if !horizontal.contains(&(&j+1)){
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
