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

## Initialize directory and create files

First of all, run qdo init to generate .qdo folder under your home dir.

```bash
qdo init
```

qdo automatically generate files run by qdo create command The files are all stored in the directory with a yyyy/mm/dd/ format. qdo uses UUID ver. 4 as file name by default. The ID can be used to manage your quiz score.

```bash
qdo create
```

generates the file like 45846ee5-a563-4f5e-a9dd-4a5a425a9095.html.

You have to save the specific directory, just add the path option.

```bash
qdo create --path ./quiz
```

In this case, the file is generated under the quiz folder at the current directory.

## User specific template

Normally, qdo generates a html file with a default template. The users can also use the their own template with the --template option. 

```bash
qdo create --template your_template.html
```

It can also generates the file from a specific template and saves it to a spcific directory.

```bash
qdo create --template your_template.html --path ./quiz
```

## Small quiz is better

A quiz should be created as small as possible. You create small quizzes as many as you need if you want to test the complex concept.

A small quiz makes it possible to test in a quicker iteration. It's important to keep learner's motivation. Some quizzes dependent on other quizzes and that relation guide the learners what they solve next or what they have to learn further.

René Descartes, in concise, said that:

> Divide each difficulty into as many parts as is feasible and necessary to resolve it.

Hint: You should avoid a long answer. If you write a long sentence in the answer part, it's a sign to divide a quiz. You try to look for index words and split them up into other quizzes.

## How to write a quiz?

### meta data

A file containes some meta data to describe the information

| 名前 | 説明 |
| --- | --- |
| generator | クイズを生成したアプリケーション, e.g. qdo|
| keywords | タグ |
| timestamp | 生成日時 |

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
        <title>タイトル</title>
    </head>

    <body>
        <header>
            <h1 class="title">タイトル</h1>
        </header>

        <main class="quiz">
            <section class="question">
                <p>質問です。</p>
            </section>

            <section class="answer">
                <p>解答です。</p>
            </section>
        </main>
    </body>
</html>
```

## TODO

- [ ] default template
  - [ ] mark: for index list
  - [ ] select: multiple choise quiz
- [ ] user option
- [ ] config files
  - [ ] description: caregory
  - [ ] user name

## References

- [HTML elements reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
- [Basic HTML Meta Tags](https://gist.github.com/whitingx/3840905)
- [HTML5で \<blockquote\> の引用元をどう表現すればいいのか](https://note.kiriukun.com/entry/20190814-how-to-explain-blockquote-source-in-html5)