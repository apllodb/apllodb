# XQL文法定義

XQL (TODO 名前決める) の文法を、 PEG (Parsing Expression Grammar) のRust製パーサ [pest](https://github.com/pest-parser/pest) がパースできる文法で記述する。

PEGやpestに詳しくなければ、 [Grammars - A thoughtful introduction to the pest parser](https://pest.rs/book/grammars/grammars.html) を読めば一通り把握できる。

## Why pest?

Rustコードが生成できるパーサジェネレータとしては、pest以外にも nom や combine も著名だが、Rustが読めない人でも文法定義は読めるようにしたく、PEGベースのpestを使う。
