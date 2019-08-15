# アッカーマン関数の計算過程を表示する(その↑)のコード片

本リポジトリでは、技術書典7で頒布した「アッカーマン関数の計算過程を表示する(その↑)」本のコード片を提供しています。

主に`src/bin`フォルダに格納されていますので、下記対応表を見て目的のファイルを探して下さい。


## 3章

- [ナイーブなアッカーマン関数の計算 - 「リスト 3.5: メカニカルなテスト」 付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/calc_naive.rs)
- [スレッド化したアッカーマン - 「リスト 3.11: thread を使う ackermann」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/calc_naive_threaded.rs)
- [多倍長整数を使ったアッカーマン - 「リスト 3.17: clone した m を渡す」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/calc_naive_with_big_number.rs)
- [メモ化したアッカーマン - 「リスト 3.24: cfg を利用したベンチマーク専用コード」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/calc_memo_with_big_number.rs)
- [計算に特化したアッカーマン - 「リスト 3.30: 結果式とパターンマッチを利用した ackermann」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/calc_math.rs)

## 4章

- [文字列結合のベンチマーク - 「リスト 4.14: 文字列結合のベンチマーク(join)」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/print_naive.rs)
- [計算過程を吐くアッカーマン - 「リスト 4.19: 入力に対応した main 関数」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/print_naive.rs)
- [メモ化して計算過程を吐くアッカーマン - 「リスト 4.21: メモ化して過程を吐く ackermann」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/print_memorized.rs)
- [メモ化して配列表現を使って計算過程を吐くアッカーマン - 「リスト 4.27: 数値の二次元配列から文字列に変換する prettier」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/print_ary.rs)
- [メモ化をせず、配列表現を使って計算過程を吐くアッカーマン - 「リスト 4.27: 数値の二次元配列から文字列に変換する prettier」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/print_ary_no_memo.rs)
- [配列表現を吐くアッカーマン - 「リスト 4.28: 中間データ表現」付近](https://github.com/esplo/ackprinter-book-supplements/blob/master/src/bin/print_ary_no_memo_no_prettier.rs)
