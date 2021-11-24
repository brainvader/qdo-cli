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

## How to use

Simply run qdo with your quiz title:

```bash
qdo --title your-quiz 
```

This generate quiz file named with timestamp like 120917.html. 

It also generates a folder structure that has a format quiz/yyyy/mm/dd under the folder you run qdo.

Just want check the output:

```bash
qdo --dry-run --title your-quiz
```

You can see what it'll generate.

## Small quiz is better

A quiz should be created as small as possible. You create small quizzes as many as you need if you want to test the complex concept.

A small quiz makes it possible to test in quick iteration. It's important keep learners motivation. Some quizzes dependent on other quizzes and that relation guide the learners what they solve next or what they have to learn.

René Descartes,  in concise, said that:

> Divide each difficulty into as many parts as is feasible and necessary to resolve it.

Hint: You should avoid a long answer. If you write a long sentence in the answer part, it's a sign to divide a quiz. You try to look for index words and split them up into other quizzes.

## How to write a quiz?

### meta data

| 名前 | 説明 |
| --- | --- |
| generator | クイズを生成したアプリケーション, e.g. qdo-cli|
| keyword | CSV形式のタグ |
| author | 著者名 |
| date | 記事作成日 |

### data属性

HTMLをデータソースとして使うという観点からidやclassよりもdata属性を使った方がデータの意味を明確にできると思います. 記述も簡潔になります. これはWeb UIを機能として追加するときに有用と考えます.

セマンティックなHTML要素はReact側で指定することにします. ただしSSGの過程で変化しない, そのまま使われる要素の場合はべた書きするようにしたいと思います.

| 名前 | 説明 | 対応するHTML要素 |
| --- | --- | --- |
| data-index-word | 見出し語となる用語 | dfn, abbr |
| data-dep | そのクイズを理解するのに必要なクイズ, 依存性 | span, span |
| ruby | 英語あるいは日本など別の読み方 | rp, rt |

### Table

```html
<table>
    <caption>Table Example</caption>
    <thead>
        <tr>
            <th scope="col">
                <!-- empty cell -->
            </th>
            <th scope="col">Column 1</th>
            <th scope="col">Column 2</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <th scope="row">ROW 1</th>
            <td>field 1-1</td>
            <td>field 1-2</td>
        </tr>
        <tr>
            <th scope="row">ROW 2</th>
            <td>field 2-1</td>
            <td>field 2-2</td>
        </tr>
    </tbody>
    <tfoot>
        <tr>
            <th scope="row">ROW 3</th>
            <td>Foot 1</td>
            <td>Foot 2 </td>
        </tr>
    </tfoot>
</table>
```

### Quiz Document Structure

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


### About CSS Classes

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