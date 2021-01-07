import React from 'react'
import { useRecoilState } from 'recoil'
import styled from 'styled-components'
import { painterState } from '../state'
import { InputNumber, Select } from 'antd'
import { ELEMENTS } from '../state/constants'

const { Option } = Select

const Container = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;

  > div {
    margin-top: 10px;
  }
`

const PainterControls = () => {
  const [painter, setPainter] = useRecoilState(painterState)

  const onChange = (key, value) =>
    setPainter({
      ...painter,
      [key]: value,
    })

  return (
    <Container>
      <h2>Painter</h2>
      <span>Element</span>
      <Select
        defaultValue={painter.type}
        onChange={(value) => onChange('type', value)}
      >
        {Object.keys(ELEMENTS).map((key) => (
          <Option value={ELEMENTS[key]} key={`Option_${key}`}>
            {key}
          </Option>
        ))}
      </Select>
      <span>Brush size</span>
      <InputNumber
        value={painter.brushSize}
        onChange={(value) => onChange('brushSize', value)}
      />
    </Container>
  )
}

export default PainterControls
