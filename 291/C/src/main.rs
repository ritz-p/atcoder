use proconio::input;
use std::collections::HashMap;
// unwrap() を何回も使うと遅い
fn main(){
    input!{
        n: usize,
        s: String,
    };
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let a = s.chars().collect::<Vec<char>>();
    let mut hash: HashMap<(i32, i32), bool> = HashMap::new();
    hash.insert((0,0),true);
    for i in 0..n{
        if a[i] == 'R'{
            x += 1;
        }else if a[i] == 'L'{
            x -= 1;
        }else if a[i] == 'U'{
            y += 1;
        }else if a[i] == 'D'{
            y -= 1;
        }

        let val = hash.get(&(x,y));
        match val{
            Some(true) => {
                println!("Yes");
                return;
            },
            _ => {hash.insert((x,y),true);},
        };
    }
    println!("No");
}