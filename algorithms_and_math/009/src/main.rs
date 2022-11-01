use proconio::input;

fn main(){
    input!{
        n: usize,
        s: usize,
        arr: [usize;n],
    };
    let mut dp = vec![false; s + 1];
    dp[0] = true;
    // v in arr は 配列内の要素を参照
    for element in arr
    {
        println!("element={}",element);
        // rev()は配列の反対から
        for i in (0..s).rev()
        {
            if dp[i] && i + element <= s
            {
                println!("i+element={}+{}",i,element);
                dp[i + element] = true;
            }
        }
    }
    let r = match dp.last().unwrap()
    {
        true => "Yes",
        false => "No"
    };
    println!("{}", r);
}