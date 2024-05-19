use proconio::input;

fn main(){
    input!{
        n: usize,
        l: isize,
        r: isize,
        a: [isize;n]
    };
    let res:Vec<isize> = a.into_iter().map(|ai| ai.max(l).min(r)).collect();
    for i in res{
        print!("{} ",i);
    }
}
