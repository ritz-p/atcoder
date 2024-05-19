use proconio::input;
fn main(){
    input!{
        n: usize,
        mut a: [usize;n]
    };

    let mut pos = vec![0;n];
    let mut q:Vec<(usize,usize)> = vec![];
    for i in 0..n{
        pos[a[i]-1] = i;
    }

    for i in 0..n{
        if i+1 == a[i]{
            continue;
        }
        let tmp = pos[i];
        pos[i] = i;
        pos[a[i]-1] = tmp;
        a.swap(i,tmp);
        q.push((i,tmp));
    }
    println!("{}",q.len());
    for (x,y) in q{
        println!("{} {}",x+1,y+1);
    }
}
