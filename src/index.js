import React, { useEffect, useState } from 'react'
import ReactDOM from 'react-dom'
import App from './App'

class Root extends React.PureComponent {
  constructor(props) {
    super(props)

    this.state = {
      canvas: null,
      wasm: null,
    }
  }

  async componentDidMount() {
    try {
      let wasm = await import('./simulator/build')
      console.log(wasm)
      const universe = await wasm.Universe.new(
        Math.floor(1920 / 15),
        Math.floor(1080 / 15),
        0
      )
      this.setState({
        wasm,
        universe,
        canvas: document.getElementById('canvas'),
      })
    } catch(e) {
      console.error(e)
    }
  }

  render() {
    if (!this.state.wasm) {
      return <div>Loading...</div>
    }
    const { wasm, canvas, universe } = this.state

    return (
      <App universe={universe} memory={wasm.wasm_memory()} canvas={canvas} />
    )
  }
}

ReactDOM.render(
  <React.StrictMode>
    <Root />
  </React.StrictMode>,
  document.getElementById('root')
)
