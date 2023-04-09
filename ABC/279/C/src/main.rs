use proconio::input;
use proconio::marker;
fn main(){
    input!{
        h: usize,
        w: usize,
        map1: [marker::Chars;h],
        map2: [marker::Chars;h],
    };
    let mut row1 = vec![vec![];w];
    let mut row2 = vec![vec![];w];
    
    for i in 0..h{
        for j in 0..w{
            row1[j].push(map1[i][j]);
            row2[j].push(map2[i][j]);
        }
    }
    // println!("{:?}",row1);
    row1.sort_unstable();
    // println!("{:?}",row1);
    row2.sort_unstable();
    // println!("{:?}",row2);
    if row1 == row2 {
        println!("Yes");
    }else{
        println!("No");
    }
}