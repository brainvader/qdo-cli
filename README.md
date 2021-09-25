# Qdo-cli

Generate quiz for [qdo](https://github.com/brainvader/qdo) application

## How to install

Need to install cargo to build and install qdo-cli.

```bash
git clone git@github.com:brainvader/qdo-cli.git
cd qdo-cli
cargo install --path ./
qdo --version
```
## タグの書き方

### メタ・データの書き方

| 名前 | 説明 |
| --- | --- |
| keyword | CSV形式のタグ |
| author | 著者名 |
| date | 記事作成日 |

### idの説明

| 名前 | 説明 |
| --- | --- |
| quiz | メイン・コンテンツ |
| question | 問題分全体　|
| question-title | 問題文のタイトル |
| question-content | 問題文本体 |
| answer | 解説文全体　|
| answer-title | 解説文のタイトル |
| answer-content | 解説文本
| footnote | 脚注 |
| footnote-title | 脚注タイトル |
| footnote-list | 脚注リスト |
| footnote-number | 脚注の参照id |
| reference | 参考文献 |
| reference-title | 参考文献タイトル |
| reference-list | 参考文献リスト |
| ref-1 | 参考文献の参照id |


### classの説明

| 名前 | 説明 |
| --- | --- |
| text | テキスト |
| code | プログラムのコード |
| math | 数式 |
| math-inline | 文章中に埋め込まれた数式 |
| eq-number | 数式の参照id |
| footnote-anchor | 脚注のアンカー |
| related-word |  関連用語のアンカー |


## References

- [HTML elements reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
- [Basic HTML Meta Tags](https://gist.github.com/whitingx/3840905)
- [HTML5で \<blockquote\> の引用元をどう表現すればいいのか](https://note.kiriukun.com/entry/20190814-how-to-explain-blockquote-source-in-html5)