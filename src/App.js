import React, { useEffect } from 'react'
/* eslint-disable import/no-webpack-loader-syntax */
// import SimulationWorker from 'worker-loader!./simulation.worker.js'
import { renderLoop } from './render'
import { usePainter } from './hooks/paint'
import { useCanvas } from './hooks/canvas'
import { useRecoilValue } from 'recoil'
import { universeState } from './state'
import styled from 'styled-components'

import Canvas from './components/Canvas'
import PainterControls from './components/PainterControls'
import WorldControls from './components/WorldControls'
import PositionControls from './components/PositionControls'
import { useRegenerateUniverse } from './hooks/regenerateUniverse'

const Container = styled.div`
  display: flex;
  flex-direction: row;
`

const Controls = styled.div`
  width: 200px;
  display: flex;
  flex-direction: column;

  padding: 10px;
  background-color: white;
`

const App = () => {
  const { universe, memory } = useRecoilValue(universeState)
  const canvas = useCanvas()
  usePainter()
  useRegenerateUniverse()

  useEffect(() => {
    if (canvas) {
      renderLoop({
        universe,
        memory,
        canvas,
      })
    }
  }, [canvas, memory, universe])

  return (
    <Container>
      <Canvas />
      <Controls>
        <PainterControls />
        <WorldControls />
        <PositionControls />
      </Controls>
    </Container>
  )
}

export default App
