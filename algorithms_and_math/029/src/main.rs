use proconio::input;
 
fn main() {
    input! {
        n: usize,
    };
    // フィボナッチ
 
    if n == 1 {
        println!("{}", 1);
        return;
    }
    if n == 2 {
        println!("{}", 2);
        return;
    }
 
    let mut arr: Vec<i64> = vec![0; n + 1];
    arr[0] = 1;
    arr[1] = 1;
    for i in 2..=n {
        arr[i] = arr[i - 1] + arr[i - 2];
    }
 
    println!("{}", arr[n]);
}