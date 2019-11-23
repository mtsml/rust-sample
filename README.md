# rust-sample
Rustのお試しとメモ

# 環境構築
以下二つを順にインストールする。

- [Visual C++ Build Tools（即インストール）](https://visualstudio.microsoft.com/ja/thank-you-downloading-visual-studio/?sku=Community&rel=16)  
- [Rust](https://www.rust-lang.org/tools/install)

C++のビルドツールが無いとRustセットアップ時に警告される。ビルドツール無しでもRustのインストールは可能だがプログラム実行時にコンパイルエラーになるっぽい。

# TRPL(1~3)
とりあえず公式。TRPLと呼ばれているらしい。

- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
- [The Rust Programming Language（和訳）](https://doc.rust-jp.rs/book/second-edition/)

### 導入
> ここでいうスピードとは、 Rustで作れるプログラムのスピードと**ソースコードを書くスピード**のことです。

そうなんだ。

### Hello, World!
- Rustはスネークケース
- `println!`はマクロ呼び出し
- `!`はマクロを表す

### 数当てゲームをプログラムする
- **コメント以外に日本語を含めるとコンパイルに失敗することがある**
- 変数はimmutable
- `mut`をつけることでmutableになる
- `::`で関連関数を表す（静的メソッド）
- `&`は参照を表す
- **シャドーイングによって同名の変数が宣言できる??**

### 変数と可変性
- 定数は定数式にしかセットできない（それはそう）

### データ型
- Rustは静的型付き言語
- 型が一意に推論できない場合にはコンパイルエラーとなる
- 配列は全て同じ型

### 関数
- 文は値を返さないことに注意
- 式は終端に`;`を含まない

# TRPL(練習)
3章の最後に記載されていた以下のプログラムを実装してみる
>- 温度を華氏と摂氏で変換する。
>- フィボナッチ数列のn番目を生成する。
>- クリスマスキャロルの定番、"The Twelve Days of Christmas"の歌詞を、 曲の反復性を利用して出力する。

### 温度を華氏と摂氏で変換する
`cargo new --bin convert_temperatures`でプロジェクトを作成。

1. 華氏か摂氏かを入力し
2. 温度を入力し
3. 変換する

という内容をTRPLで学んだ内容やコードをつぎはぎして書いてみた。Rustというよりもアイディアや純粋なプログラミングスキルの問題だがビミョーな感じに。それと途中で警告出ていたのでメモ。

修正前のTemperatureらへんのコード
```rust
    // Temperature
    let mut temp :f64 = 0.0;
    loop {
        println!("Please input {} temperature.", mode);
        let mut i = String::new();
        io::stdin().read_line(&mut i)
            .expect("Failed to read line.");
        temp = match i.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
```
実行すると警告
```
warning: value assigned to `temp` is never read
  --> src\main.rs:21:13
   |
21 |     let mut temp :f64 = 0.0;
   |             ^^^^
   |
   = help: maybe it is overwritten before being read?
```
`temp`使ってるけどな～と思ったら初期化した`0.0`が使われてないからみたいだ（[参考](https://github.com/rust-lang/rust/issues/49171)）  
というか警告時のhelpのところに書いてあった。優秀…

TODO:gitignoreに実行ファイルなどを追加する
