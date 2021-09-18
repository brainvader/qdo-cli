# Qdo

Quiz style tuts, tips, and notes

## 描画の流れ

データソースとしてHTMLファイルを使う.

1. getStaticPropsでHTMLファイルを読み込む
2. JSDomあたりでパース
3. [dangerouslySetInnerHTML](https://reactjs.org/docs/dom-elements.html#dangerouslysetinnerhtml)で表示する.

2でkatexやshikiでのレンダリングを実行し置き換える. 必要ならJSONに変換しコンポーネントに渡す.

## ブログとしてのQdo

SSGの機能を利用して静的HTMLを出力する. 

```json
"ssg": "next build && next export"
```

この時Katexによる数式の描画やshikiによるコード・ハイライトが実行される.

## WebアプリとしてのQdo

簡単なクイズアプリとして実行する. rust側で操作するべきか?

TODO: [env-cmd](https://www.npmjs.com/package/env-cmd)を調べる


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
- [How to use diferent .env files with nextjs?](https://stackoverflow.com/questions/59462614/how-to-use-diferent-env-files-with-nextjs/61750672#61750672)
- [HTML5で \<blockquote\> の引用元をどう表現すればいいのか](https://note.kiriukun.com/entry/20190814-how-to-explain-blockquote-source-in-html5)