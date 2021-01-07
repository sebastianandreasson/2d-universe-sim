import React, { useEffect } from 'react'
/* eslint-disable import/no-webpack-loader-syntax */
// import SimulationWorker from 'worker-loader!./simulation.worker.js'
import { renderLoop } from './render'
import { usePainter } from './hooks/paint'
import { useCanvas } from './hooks/canvas'
import { useRecoilValue } from 'recoil'
import { universeState, simulationOnState } from './state'
import styled from 'styled-components'

import Canvas from './components/Canvas'
import PainterControls from './components/PainterControls'
import WorldControls from './components/WorldControls'
import PositionControls from './components/PositionControls'
import { useRegenerateUniverse } from './hooks/regenerateUniverse'
import SimulationControls from './components/SimulationControls'

const Container = styled.div`
  display: flex;
  flex-direction: row;
`

const Controls = styled.div`
  width: 200px;
  display: flex;
  flex-direction: column;

  padding: 10px;
  background-color: #191516;
  color: #c9daea;

  h2 {
    margin-top: 10px;
    color: #c9daea;
    text-align: center;
  }
`

const App = () => {
  const { universe, memory } = useRecoilValue(universeState)
  const simulationOn = useRecoilValue(simulationOnState)
  const canvas = useCanvas()
  usePainter()
  useRegenerateUniverse()

  useEffect(() => {
    let stop = () => {}
    if (canvas) {
      stop = renderLoop({
        universe,
        memory,
        canvas,
        simulationOn,
      })
    }
    return stop
  }, [canvas, memory, universe, simulationOn])

  return (
    <Container>
      <Canvas />
      <Controls>
        <PainterControls />
        <WorldControls />
        <PositionControls />
        <SimulationControls />
      </Controls>
    </Container>
  )
}

export default App
