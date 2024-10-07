use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,  
    };
    let mut a = vec![];
    for _i in 0..n{
        input!{
            l: usize,
            la: [usize;l]
        };
        a.push(la);
    }
    let mut res = 0;
    dfs(&a,0,1,n,x,&mut res);
    println!("{}",res);
}

fn dfs(a: &Vec<Vec<usize>>,current: usize,total: usize,n: usize,x: usize,res: &mut usize){
    if current == n{
        if total == x{
            *res += 1;
        }
        return;
    }
    for c in &a[current]{
        if total > x / c{
            continue;
        }
        dfs(a,current+1,total*c,n,x,res);
    }
}