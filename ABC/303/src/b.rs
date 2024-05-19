use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        arr: [[usize;n];m]
    };
    let mut flag = vec![vec![];n+1];
    let mut nl = 0;
    for i in 0..m{
        for j in 0..n-1{
            if !flag[arr[i][j]].contains(&arr[i][j+1]){
                flag[arr[i][j]].push(arr[i][j+1]);
            }
            if !flag[arr[i][j+1]].contains(&arr[i][j]){
                flag[arr[i][j+1]].push(arr[i][j]);
            }
        }
    }
    for i in 1..=n{
        for j in 1..=n{
            if !flag[i].contains(&j) && i != j{
                nl+=1;
            }
        }
    }
    println!("{}",nl/2);
}
