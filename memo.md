1. 一個だけ WA の時はループの範囲を疑う(0 とか 1 を変える)
2. 順列，組み合わせ，重複組み合わせを列挙する
   - combinations -> 数字被りなし(0,1),(1,2)
   - permutations -> 数字被りありで同じ数字が一度のみでてくる(0,1),(1,0)
   - combinations_with_replacement -> 数字被りありで同じ数字が複数回でてくる(0,0),(0,1)
3. 素数をまとめて判定する場合は以下のように考える
   ```rust
    let mut prime = vec![true; limit];
    let mut i = 2;
    loop {
        if prime[i] {
            let mut j = 2;
            while i * j < limit {
                prime[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
   ```
