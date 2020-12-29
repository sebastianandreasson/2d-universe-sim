import React from 'react'
import { useRecoilState, useRecoilValue } from 'recoil'
import styled from 'styled-components'
import { simulationOnState, universeState } from '../state'
import { Button, Switch, Space } from 'antd'

const Container = styled.div`
  width: 100%;

  display: flex;

  flex-direction: column;
`

const SimulationControls = () => {
  const { universe } = useRecoilValue(universeState)
  const [simulationOn, setSimulationOn] = useRecoilState(simulationOnState)

  return (
    <Container>
      <span>Simulation</span>
      <Button onClick={() => universe.tick()}>Tick</Button>
      <Space>
        On
        <Switch
          onChange={() => setSimulationOn(!simulationOn)}
          value={simulationOn}
        />
      </Space>
    </Container>
  )
}

export default SimulationControls
