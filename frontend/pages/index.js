import path from 'path'
import fs from 'fs'
import { JSDOM } from 'jsdom'
import katex from 'katex'

export default function Home({post}) {
  return (
    <div dangerouslySetInnerHTML={{__html: post}}></div>
  )
}

// get keywords in meta tag
function getKeywords(metaData) {
  const metaValues = Object.values(metaData);
  const keywords = metaValues.find((meta) => meta.name === "keywords");
  return keywords.content.split(",");
}

function renderMath(mathString) {
  return katex.renderToString(mathString, {
    displayMode: true,
    output: 'mathml',
    throwOnError: false
  })
}

export async function getStaticProps(context) {
  console.log('getStaticProps')

  // Read data source
  const dataDir = path.resolve(process.cwd(), '..', 'data')
  const dataFiename = path.resolve(dataDir, "sample.html")
  const dataPath = path.resolve(dataDir, dataFiename)
  const data = fs.readFileSync(dataPath, 'utf8');

  const domTree = await JSDOM.fromFile(dataPath)
  const document = domTree.window.document
  const mathNode = document.querySelector('.math')
  mathNode.innerHTML = renderMath(mathNode.textContent)

  return {
    props: {
      post: domTree.serialize()
    },
  }
}