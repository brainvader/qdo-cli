import path from 'path'
import fs from 'fs'

export default function Home({post}) {
  return (
    <div dangerouslySetInnerHTML={{__html: post}}></div>
  )
}

export async function getStaticProps(context) {
  const dataDir = path.resolve(process.cwd(), '..', 'data')
  const dataFiename = fs.readdirSync(dataDir)[1]
  const dataPath = path.resolve(dataDir, dataFiename)
  const data = fs.readFileSync(dataPath, 'utf8');
  return {
    props: {
      post: data
    },
  }
}