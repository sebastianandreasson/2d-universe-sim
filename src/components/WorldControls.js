import React from 'react'
import { useRecoilState } from 'recoil'
import styled from 'styled-components'
import { universeSettingsState } from '../state'
import { InputNumber, Slider } from 'antd'

const Container = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;

  > div {
    margin-top: 10px;
  }
`

const WorldControls = () => {
  const [settings, setSettings] = useRecoilState(universeSettingsState)

  const onChange = (key, value) =>
    setSettings({
      ...settings,
      [key]: value,
    })

  const input = (type, value, step = 1) => {
    return (
      <>
        <span>{type}</span>
        <InputNumber
          value={value}
          onChange={(v) => onChange(type, v)}
          step={step}
        />
      </>
    )
  }

  return (
    <Container>
      <h2>World</h2>
      {input('seed', settings.seed)}
      {input('octaves', settings.octaves)}
      {input('gain', settings.gain, 0.1)}
      {input('lacunarity', settings.lacunarity, 0.1)}
      {input('frequency', settings.frequency, 0.1)}
      water level
      <Slider
        min={0}
        max={1}
        step={0.05}
        defaultValue={settings.water_level}
        onChange={(v) => onChange('water_level', v)}
      />
    </Container>
  )
}

export default WorldControls
