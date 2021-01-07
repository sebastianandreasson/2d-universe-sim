import React from 'react'
import { useRecoilState } from 'recoil'
import styled from 'styled-components'
import { positionState } from '../state'
import { InputNumber } from 'antd'

const Container = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;

  > div {
    margin-top: 10px;
  }
`

const PositionControls = () => {
  const [position, setPosition] = useRecoilState(positionState)

  const onChange = (key, value) =>
    setPosition({
      ...position,
      [key]: value,
    })

  const input = (type, value) => {
    return (
      <>
        <span>{type}</span>
        <InputNumber value={value} onChange={(v) => onChange(type, v)} />
      </>
    )
  }

  return (
    <Container>
      <h2>Position</h2>
      {input('x', position.x)}
      {input('y', position.y)}
    </Container>
  )
}

export default PositionControls
