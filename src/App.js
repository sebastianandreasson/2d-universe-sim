import React, { useEffect, useState } from 'react'
/* eslint-disable import/no-webpack-loader-syntax */
// import SimulationWorker from 'worker-loader!./simulation.worker.js'
import { renderLoop } from './render'
import { usePainter } from './hooks/paint'
import { useCanvas } from './hooks/canvas'
import { useRecoilValue } from 'recoil'
import { universeState } from './state'

const App = () => {
  const { universe, memory } = useRecoilValue(universeState)
  const canvas = useCanvas()
  usePainter()

  useEffect(() => {
    if (canvas) {
      renderLoop({
        universe,
        memory,
        canvas,
      })
    }
  }, [canvas])

  return (
    <button style={{ position: 'absolute', right: 0, top: 0 }}>heyo</button>
  )
}

export default App
