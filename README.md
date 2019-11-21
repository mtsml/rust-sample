# rust-sample
Rustのお試しとメモ

# 環境構築
以下二つを順にインストールする。

- [Visual C++ Build Tools（即インストール）](https://visualstudio.microsoft.com/ja/thank-you-downloading-visual-studio/?sku=Community&rel=16)  
- [Rust](https://www.rust-lang.org/tools/install)

C++のビルドツールが無いとRustセットアップ時に警告される。ビルドツール無しでもRustのインストールは可能だがプログラム実行時にコンパイルエラーになるっぽい。

# TRPL
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

