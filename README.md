# 型や関数メモ

## permutation

### permutations

    - 被りありで同じ値が一度のみでてくる(0,1),(1,0)

    ```rust
    v.permutation(v.len())
    ```

### permutation().unique()

    - (0,0,1,1) などで同じ値が複数あるときに並びが同じとき重複をなくす

    ```
    (0,0,1,1)
    (0,0,1,1) <- ここの重複をなくす
    (0,1,0,1)
    ```

    ```rust
    v.permutation(v.len()).unique()
    ```

## f64

### floor

    - 小数点以下切り捨て

    ```rust
    let mut f = 0.9;
    f = f.floor();
    ```

## 標準出力

    - 標準出力する

    ```rust
    let s = 100;
    println!("{}",res);
    ```

    - 小数点以下の桁数指定での標準出力

    ```rust
    let res = 0.0001;
    println!("{:.4}",res);
    ```
    　- 四捨五入されるので必要に応じて floor など使う

## HashSet

### new()

    - 空の HashSet を作る

    ```
    let mut set = HashSet::new();
    ```

### from_iter()

    - Vec から HashSet を生成する

    ```rust
    let v = vec![0,1,2];
    let set: HashSet<usize> = HashSet::from_iter(v);
    ```
