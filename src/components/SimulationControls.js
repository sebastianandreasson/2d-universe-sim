import React from 'react'
import { useRecoilState, useRecoilValue } from 'recoil'
import styled from 'styled-components'
import { simulationOnState, universeState } from '../state'
import { Button, Switch, Space } from 'antd'

const Container = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;

  > div {
    margin-top: 10px;
  }
`

const SimulationControls = () => {
  const { universe } = useRecoilValue(universeState)
  const [simulationOn, setSimulationOn] = useRecoilState(simulationOnState)

  return (
    <Container>
      <h2>Physics</h2>
      <Space style={{ marginBottom: 10 }}>
        On
        <Switch
          onChange={() => setSimulationOn(!simulationOn)}
          checked={simulationOn}
        />
      </Space>
      <Button onClick={() => universe.tick()}>Tick</Button>
    </Container>
  )
}

export default SimulationControls
