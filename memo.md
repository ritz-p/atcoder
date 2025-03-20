## 一つだけ WA の場合疑うこと

1. ループの範囲を疑う(i=0 とか i=1 を変える)
2. HashMap などで i32 などの型が暗黙的に充てられてオーバーフローを起こすため usize などを指定
3. 初期値について考え直す(usize::MAX でいいか 0 でいいかなど)

## 順列、組み合わせを列挙する

1. combinations -> 数字被りなし(0,1),(1,2)
2. permutations -> 数字被りありで同じ数字が一度のみでてくる(0,1),(1,0)
3. combinations_with_replacement -> 数字被りありで同じ数字が複数回でてくる(0,0),(0,1)

## 素数をまとめて判定したいとき

1. 以下のように書く

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

## 公差が 0 未満の場合初項

1. 公差が 0 未満の場合初項を最低値にして足していく
   ```rust
   if d < 0 {
       a = a + d * (n - 1);
       d *= -1;
   }
   ```

## 二部探索(binary_search について)

1. binary_search
   ```rust
   let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
   assert*eq!(s.binary_search(&13), Ok(9));
   assert_eq!(s.binary_search(&4), Err(7));
   assert_eq!(s.binary_search(&100), Err(13));
   let r = s.binary_search(&1);
   assert!(match r { Ok(1..=4) => true, * => false, });
   ```

## ビット全探索について

1. 配列の分割方法を記述する場合

   ```rust
   for i in 0..(1 << n){
       for j in 0..n{
           if i >> j & 1 != 0{
            // 処理
           }
       }
   }
   ```

   - 2 進数が n 桁あると考えて、n の k 桁目が 0 のときに分割するイメージ

     ```
     0000 -> 分割なし
     0001 -> 4,3,2 と 1 で分割
     0011 -> 4,3,2 と 1 で分割
     0101 -> 4,3 と 2,1 で分割
     0111 -> 4,3 と 2 と 1 で分割
     1001 -> 4 と 3,2,1 で分割
     1100 -> 4 と 3 と 2,1 で分割
     1110 -> 4 と 3 と 2,1 で分割
     1111 -> 4 と 3 と 2 と 1 で分割
     ```

## 単純無向グラフの経路の全探索について

1. start から goal までの経路を探索する

   ```rust
   fn dfs(
   start: usize,
   goal: usize,
   visited: &mut Vec<bool>,
   graph: &Vec<Vec<(usize, usize)>>,
   ) {
   if start == goal {
        // なんか処理
       return;
   }
   for &(next, w) in &graph[start] {
       if !visited[next] {
           visited[next] = true;
           dfs(next, goal, visited, graph);
           visited[next] = false;
       }
   }
   }
   ```
