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

  componentDidMount() {
    import('./simulator/build').then((wasm) => {
      const universe = wasm.Universe.new(
        Math.floor(1920 / 10),
        Math.floor(1080 / 10),
        0
      )
      this.setState({
        wasm,
        universe,
        canvas: document.getElementById('canvas'),
      })
    })
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
