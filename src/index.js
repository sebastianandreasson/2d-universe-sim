import React, { useEffect } from 'react'
import { RecoilRoot, useRecoilState, useRecoilValue } from 'recoil'
import ReactDOM from 'react-dom'
import App from './App'
import { universeSettingsState, universeState, wasmState } from './state'
import 'antd/dist/antd.css'

const Root = () => {
  const [wasm, setWasm] = useRecoilState(wasmState)
  const { width, height } = useRecoilValue(universeSettingsState)
  const [universe, setUniverse] = useRecoilState(universeState)

  useEffect(() => {
    const run = async () => {
      try {
        let _wasm = await import('./simulator/build')
        setWasm(_wasm)
      } catch (e) {
        console.error(e)
      }
    }
    run()
  }, [setWasm])
  useEffect(() => {
    if (!wasm) return
    const u = wasm.Universe.new(width, height, 0)
    setUniverse({
      universe: u,
      memory: wasm.wasm_memory(),
    })
  }, [wasm, width, height, setUniverse])

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
