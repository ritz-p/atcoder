use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main(){
    input!{
        h: usize,
        w: usize,
        mut m: [Chars;h]
    };
    let mut loopable = true;
    let mut visited = HashSet::new();
    let mut visited_v = HashSet::new();
    loop{
        loopable = false;
        let mut row = vec![];
        for i in 0..h{
            if visited.contains(&i){
                continue;
            }
            let mut horizontal_hash = HashSet::new();
            let mut remains = 0;
            for j in 0..w{
                if m[i][j] != '.'{
                    horizontal_hash.insert(m[i][j]);
                    remains += 1;
                }
            }
            if horizontal_hash.len() == 1 && remains >= 2{
                row.push(i);
                visited.insert(i);
                loopable = true;
            }
        }
        for i in 0..w{
            let mut vertical_hash = HashSet::new();
            let mut remains = 0;
            if visited_v.contains(&i){
                continue;
            }
            for j in 0..h{
                if m[j][i] != '.'{
                    vertical_hash.insert(m[j][i]);
                    remains += 1;
                }
            }
            if vertical_hash.len() == 1 && remains >= 2{
                for j in 0..h{
                    m[j][i] = '.';
                }
                visited_v.insert(i);
                loopable = true;
            }
        }
        for i in row{
            for j in 0..w{
                m[i][j] = '.';
            }
        }
        if !loopable{
            break;
        }
    }
    let mut res = 0;
    for i in 0..h{
        for j in 0..w{
            if m[i][j] != '.'{
                res += 1;
            }
        }
    }
    println!("{}",res);
}
