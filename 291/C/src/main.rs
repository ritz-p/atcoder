use proconio::input;

fn main(){
    input!{
        n: usize,
        s: String,
    };
    let mut x:isize = 0;
    let mut y:isize = 0;
    let mut xy:Vec<(isize, isize)> = vec![(0,0)];
  	let a = s.chars().collect::<Vec<char>>();
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
        xy.push((x,y));
    }
    xy.sort_unstable();
    xy.dedup();
    if xy.len() < n+1{
        println!("Yes");
    }else{
        println!("No"); 
    }
}