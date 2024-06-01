use proconio::input;

fn main(){
    input!{
        n: usize,
        l: usize,
        r: usize,
    };
    for i in 1..l{
        print!("{} ",i);
    }
    for i in (l..=r).rev(){
        print!("{} ",i);
    }
    for i in r+1..=n{
        print!("{} ",i);
    }
}
