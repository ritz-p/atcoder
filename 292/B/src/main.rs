use proconio::input;

fn main(){
    input!{
        n: usize,
        q: usize,
        tup: [(usize,usize);q],
    };
    let mut players:Vec<(usize,usize)> = vec![];
    for i in 0..n{
        players.push((i,0));
    }
    for (order,p) in tup{
        if order == 1{
            players[p-1].1 += 1;
        }else if order == 2{
            players[p-1].1 += 2;
        }else if order == 3{
            if players[p-1].1 >= 2{
                println!("Yes");
            }else{
                println!("No");
            }
        }
    }
}