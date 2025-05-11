# 関数メモ

## permutation

### permutations

    - 被りありで同じ値が一度のみでてくる(0,1),(1,0)
    - 使い方

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

    - 使い方

    ```rust
    v.permutation(v.len()).unique()
    ```
