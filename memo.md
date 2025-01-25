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
4. 公差が 0 未満の場合初項を最低値にして足していく
   ```rust
   if d < 0 {
       a = a + d * (n - 1);
       d *= -1;
   }
   ```
5. binary_search
   ```rust
   let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
   assert*eq!(s.binary_search(&13), Ok(9));
   assert_eq!(s.binary_search(&4), Err(7));
   assert_eq!(s.binary_search(&100), Err(13));
   let r = s.binary_search(&1);
   assert!(match r { Ok(1..=4) => true, * => false, });
   ```
