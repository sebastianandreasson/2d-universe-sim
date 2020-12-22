import React, { useEffect } from 'react'
import { RecoilRoot, useRecoilState, useRecoilValue } from 'recoil'
import ReactDOM from 'react-dom'
import App from './App'
import {
  positionState,
  universeSettingsState,
  universeState,
  wasmState,
} from './state'
import 'antd/dist/antd.css'

const Root = () => {
  const [wasm, setWasm] = useRecoilState(wasmState)
  const [universe, setUniverse] = useRecoilState(universeState)
  const settings = useRecoilValue(universeSettingsState)
  const position = useRecoilValue(positionState)

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
  }, [])
  useEffect(() => {
    if (!wasm) return
    const universeSettings = wasm.UniverseSettings.new(
      settings.seed,
      settings.octaves,
      settings.gain,
      settings.lacunarity,
      settings.frequency
    )
    const u = wasm.Universe.new(
      settings.width,
      settings.height,
      universeSettings,
      universeSettings.seed
    )
    setUniverse({
      universe: u,
      memory: wasm.wasm_memory(),
    })
  }, [wasm])

  useEffect(() => {
    if (!universe) return
    const pos = wasm.Position.new(position.x, position.y)
    const universeSettings = wasm.UniverseSettings.new(
      settings.seed,
      settings.octaves,
      settings.gain,
      settings.lacunarity,
      settings.frequency
    )
    universe.universe.regenerate(universeSettings, pos)
  }, [universe, settings, position])

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
