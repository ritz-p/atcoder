use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main(){
    input!{
        h: usize,
        w: usize,
        s: [Chars;h]
    };
    let mut res = 0;
    let mut visited:HashSet<(usize,usize)> = HashSet::new();
    for i in 0..h{
        for j in 0..w{
            if s[i][j] == '#'{
                if !visited.contains(&(i,j)){
                    res += 1;
                }
                dfs(&s,(i,j),&mut visited,h,w);
            }
        }
    }
    println!("{}",res);
}

fn dfs(grid: &Vec<Vec<char>>,pos:(usize,usize),visited:&mut HashSet<(usize,usize)>,xmax: usize,ymax: usize){
    if visited.contains(&pos){
        return
    }
    let mut positions:Vec<(usize,usize)> = vec![];
    if pos.0 > 0{
        if grid[pos.0-1][pos.1] == '#'{
            positions.push((pos.0-1,pos.1));
        }
        if pos.1 > 0{
            if grid[pos.0-1][pos.1-1] == '#'{
                positions.push((pos.0-1,pos.1-1));
            }
        }
        if pos.1 < ymax-1{
            if grid[pos.0-1][pos.1+1] == '#'{
                positions.push((pos.0-1,pos.1+1));
            }
        }
    }
    if pos.0 < xmax-1{
        if grid[pos.0+1][pos.1] == '#'{
            positions.push((pos.0+1,pos.1));
        }
        if pos.1 > 0{
            if grid[pos.0+1][pos.1-1] == '#'{
                positions.push((pos.0+1,pos.1-1));
            }
        }
        if pos.1 < ymax-1{
            if grid[pos.0+1][pos.1+1] == '#'{
                positions.push((pos.0+1,pos.1+1));
            }
        }
    }
    if pos.1 > 0{
        if grid[pos.0][pos.1-1] == '#'{
            positions.push((pos.0,pos.1-1));
        }
    }
    if pos.1 < ymax-1{
        if grid[pos.0][pos.1+1] == '#'{
            positions.push((pos.0,pos.1+1));
        }
    }
    for position in positions{
        visited.insert(pos);
        dfs(grid,position,visited,xmax,ymax);
    }
}