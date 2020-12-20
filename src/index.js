import React, { useEffect } from 'react'
import { RecoilRoot, useRecoilState } from 'recoil'
import ReactDOM from 'react-dom'
import App from './App'
import { universeState } from './state'

const Root = () => {
  const [universe, setUniverse] = useRecoilState(universeState)
  useEffect(() => {
    const run = async () => {
      try {
        let wasm = await import('./simulator/build')
        const u = await wasm.Universe.new(
          Math.floor(1920 / 15),
          Math.floor(1080 / 15),
          0
        )
        setUniverse({
          universe: u,
          memory: wasm.wasm_memory(),
        })
      } catch (e) {
        console.error(e)
      }
    }
    run()
  }, [])

  if (!universe) {
    return <div>Loading...</div>
  }

  return <App />
}

ReactDOM.render(
  <React.StrictMode>
    <RecoilRoot>
      <Root />
    </RecoilRoot>
  </React.StrictMode>,
  document.getElementById('root')
)
