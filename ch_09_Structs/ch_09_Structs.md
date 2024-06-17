# Structs

Rustの構造体（Struct）には3種類ある．

+ Named-field
    + フツウの構造体（他の言語と同じ）
+ Tuple-like
    + タプルのような構造体
+ Unit-like
    + 型名だけで中身がない（空）の構造体

## Named-field Structs

いわゆる普通の構造体（要素を名前でアクセス）

```rust
struct GrayscaleMap {
    pixels: Vec<u8>,    // 要素の区切りはカンマ(,) (TypeScript（セミコロン)とは違う）
    size: (usize, usize)
}

let width = 1024;
let height = 576;
let image = GrayscaleMap {
    pixels: vec![0; width * height],
    size: (width, height)
};
```

Rustの慣例では、型名はキャメルケース、フィールドやメソッド名はスネークケースで書く

構造体のメンバはデフォルトではprivate．外部から触りたい場合は`pub`を付ける．

```rust
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}
```

構造体自体は`pub`にするが、メンバを`private`にしておくことも（もちろん）可能．

```rust
pub struct GrayscaleMap {
    pixels: Vec<u8>,    // private
    size: (usize, usize)    // private
}
```

「.. EXPR」の記法で、既存の構造体のメンバを一括で取り出せる．
（これもJavaScriptなどと同じ）

```rust
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }  // Copy, Cloneを自動導出してコピーできるようにする

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height/2, .. b };
        // heightメンバ以外はbから引き継ぐ
        // b.nameの所有権はbroom1に移る（StringはCopyトレイトを持っていないため）
    let mut broom2 = Broom { name: b.name.clone(), .. broom1}
        // StringはCopyトレイトを持っていないので、明示的にclone()で名前のコピーを作る．
        // nameメンバ以外はbroom1から引き継ぐ
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)    // 返り値
}
```


## Tuple-like Structs

タプルのような構造体

```rust
struct Bounds(usize, usize) // {}でなく()
```

例）

```rust
let image_bounds = Bounds(1024, 768);
let total_size = image_bounds.0 * image_bounds.1;   // 要素へのアクセス方法（C#と同じ）
```

Tuple-like構造体は、既存の型に新しい型名を付ける時などに使われる（コンパイラによる厳密な型チェックを使いたい時など）
（Typeでいいような気もするが・・）

例）
```rust
struct Ascii(Vec<u8>);  // Vec<u8>の代わりにAsciiというハッキリした名前を付ける．
```

## Unit-like Structs

要素が何もない構造体

```rust
struct Onesuch;     // 要素なし
```

ユニット型()と同じく、メモリを全く消費しない（サイズ0）

unit-like構造体はトレイト使用時に有用になることがある．

## Struct Layout

C/C++と異なり、Rustの構造体はメンバの格納順序は保証しない（どう配置されるかはRustまかせ）

一方、構造体のメンバの値は、メモリ上に直接配置されることは保証される．
（javaScriptやpythonなどの言語は単純な数値型でもヒープに配置して、そこへのポインタを格納する）
単純型（u8やf64など）はそのまま構造体のフィールドに、文字列型(String)などは、データはヒープに格納され、
構造体のメンバはそこへのポインタを持つ、という形になる．
（C#も確かこうなっていたはず）

メモリ上の配置の仕方を、C/C++のようにするオプション(`#[repr(C)]`)もある．
これを使うと、構造体のメンバの値は、定義の順序通りにメモリに配置される．

## Defining methods with impl

`impl`キーワードで構造体のメソッドを定義できる．

Rustでは、メソッドのことをAssociated Functionとも言う（型に連結している関数、の意図だろう）

一方、普通の（単独の）関数はFree Functionという．

Rustのメソッドは、第1引数に必ず`self`を持つ（pythonと同じノリ）

第1引数に`self`を取らないメソッドを定義することも可能．これは`static method`と呼ばれる．
（多言語と同じ）

`static method`は通常、コンストラクタとして使われる．

```rust
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>
}

impl Queue {
    // newの第1引数にselfがないので、これはstatic method．
    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}
```

コンストラクタの名前は`new`が慣例だが、他の任意の名前でも良い．

## Genetic Structs

構造体の定義でジェネリックを使える．

```rust
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> for Queue<T> {
    pub fn new() -> Self {  // Selfは、このメソッドの定義対象の構造体を指す（ここではQueue<T>と同じ）
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.younger.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}
```

## Structs with Lifetime parameters

構造体に、他の要素への参照を含める場合は、生存期間パラメータ（lifetime parameter）の指定が必要になる．

（サンプルコードを見たほうが早いのでそっちを参照のこと）

## Deriving Common Traits for Struct Types

よく必要になるトレイトは、`#[derive]`属性で自動導出できる．

例）
```rust
struct Point {
    x: f64,
    y: f64
}
```

このままでは、Point構造体のコピー、クローン、Point同士の比較、デバッグ用出力ができない．
#deriveを使えばこれらの機能を自動導出できる．

```rust
#[derive(Copy, Clone, Debug, PartialEq)]    // 常にこれらを付けておけば良い．
struct Point {
    x: f64,
    y: f64
}
```

## Interior Mutability

構造体内部の一部のメンバだけをmutableにする方法

（ちょっとこの章は意味がよくわからなかった・・（ので保留））(^^;)
