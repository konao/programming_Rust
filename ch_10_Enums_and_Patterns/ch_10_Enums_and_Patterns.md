# Enums and Patterns

## Enums

## Enums with Data

## Enums in Memory

## Rich Data Structures Using Enums

## Generic Enums

## Patterns

## Literals, Variables, and Wildcards in Patterns

## Tuple and Struct Patterns

## Reference Patterns

matchの条件部分に参照が与えられた場合の話．

特に、条件部分で構造体などのdestrucuringが起こる場合、（Rustは他言語と異なり）値がmoveされるか、copyされるか、
参照で受け取るか、が重要になってくるのでこういうメンドクサイ話が出てくる．

例）以下のような場合、`name`, `language`の値はmoveされる．

```rust
match account {
    Account { name, language, .. } => {
        // name, languageはaccountからmoveされたものになる
    }
}
```

moveしたくない場合は、`ref name`, `ref language`のように、構造体メンバ名の前に`ref`を付ける．

*（ここに&が付けられればいいのに・・（統一性がないよな））*

```rust
match account {
    Account { ref name, ref language, .. } => {
        // こうすると、name, languageはaccountのメンバの参照となる．
    }
}
```

同様に、matchの引数に参照が来た場合、条件部分に&を付ける必要がある．

```rust
match sphere.center() { // center()メソッドはsphereの保持しているデータ（中心座標）の参照を返してくるものとする
    &Point3d { x, y, z } => {   // Point3dの前に&が必要
        ...
    }
}
```

上記でx, y, zがcopyableでないときは、`ref`キーワードを付ける必要がある．

```rust
match sphere.center() { // center()メソッドはsphereの保持しているデータ（中心座標）の参照を返してくるものとする
    &Point3d { ref x, ref y, ref z } => {
        ...
    }
}
```

## Matching Multiple Possibilities

## Pattern Guards

## @ patterns

マッチしたパターンを、そのまま処理部で使いたい時に@を使う．

例）
```rust
match self.get_selection() {
    Shape::Rect(top_left, bottom_right) => 
        optimized_paint(&Shape::Rect(top_left, bottom_right)),  // (1)
    other_shape =>
        paint_outline(other_shape.get_outline())
}
```
上のコードでは、(1)の部分で、条件部でマッチしたときに取り出される値(top_left, bottom_right)を
使ってまた同じ値(=Shape::Rect(top_left, bottom_right))を作っている．

このような場合は@を使うと、条件部にマッチした値をそのまま処理部で使える．

```rust
match self.get_selection() {
    rect @ Shape::Rect(top_left, bottom_right) => 
        optimized_paint(&rect),
    other_shape =>
        paint_outline(other_shape.get_outline())
}
```

また、条件部でrangeにマッチさせたいとき（そしてその値を処理部で使いたいとき）も有用である．

```rust
match chars.next() {
    Some(digit @ '0' ... '9') => read_number(digit, chars),
    ...
}
```

## Where Patterns Are Allowed

## Populating a Binary Tree
