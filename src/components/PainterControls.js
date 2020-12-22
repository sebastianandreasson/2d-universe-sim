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
      <span>Painter</span>
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
      <InputNumber
        value={painter.brushSize}
        onChange={(value) => onChange('brushSize', value)}
      />
    </Container>
  )
}

export default PainterControls
