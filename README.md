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

### Quiz Document Structure

```html
<!DOCTYPE html>
<html lang="en">

    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible"
            content="IE=edge">
        <meta name="viewport"
            content="width=device-width, initial-scale=1.0">
        <meta name="generator"
            content="{{ generator }}" />
        <meta name="keywords"
            content="コンピュータ・ネットワーク" />
        <meta name="author"
            content="brainvader" />
        <meta name='date'
            content="{{ timestamp }}">
        <meta name="description"
            content="ノードの定義ファイル" />
        <title>{{ title }}</title>
    </head>

    <body>
        <header>
            <h1 class="title">{{ title }}</h1>
        </header>

        <main class="quiz">
            <section class="question">
                <p>クイズとは何ですか?</p>
            </section>

            <section class="answer">
                <p>クイズの解答を書きます. </p>
                <p>用語の定義は以下のようにします.</p>
                <p><dfn>React</dfn>はFacebook社が開発しているUIライブラリです.
                    あるいは略語なら<dfn><abbr
                            title="HyperText Markup Language">HTML</abbr></dfn>のようにします.
                    必要ならid属性を付与してページ内から参照できるようにしてもいいでしょう.
                    しかし通常はクイズの粒度は小さくして,
                    リンクは外部の別のクイズを指す方が適切です.
                </p>
                <p>クイズはいくつかの表記をサポートしています.　一つはハイライトされたコードです.
                </p>
                <pre
                    class="code"><code>const a = 10</code></pre>
                <p>場合によっては数式を表示できる必要もあるでしょう.</p>
                <div class="math"
                    id="eq-1">
                    \tag{1} \frac{a}{b} = 10
                </div>
                <p>ページ内に登場する数式への参照<a
                        href="#eq-1">(1)</a>はこんな感じになります.
                </p>
                <p>インラインは<span
                        class="math-inline">a'</span>と書けます.
                </p>
            </section>
        </main>

        <aside class="dependencies">
            <ul class="list">
                <li class="relationship"><a href="/">Quiz
                        X</a></li>
            </ul>
        </aside>
    </body>

</html>
```

## References

- [HTML elements reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
- [Basic HTML Meta Tags](https://gist.github.com/whitingx/3840905)
- [HTML5で \<blockquote\> の引用元をどう表現すればいいのか](https://note.kiriukun.com/entry/20190814-how-to-explain-blockquote-source-in-html5)