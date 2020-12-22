import React from 'react'
import { useRecoilState } from 'recoil'
import styled from 'styled-components'
import { universeSettingsState } from '../state'
import { InputNumber } from 'antd'

const Container = styled.div`
  width: 100%;

  display: flex;

  flex-direction: column;
`

const WorldControls = () => {
  const [settings, setSettings] = useRecoilState(universeSettingsState)

  const onChange = (key, value) =>
    setSettings({
      ...settings,
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
      <span>World</span>
      {input('seed', settings.seed)}
      {input('octaves', settings.octaves)}
      {input('gain', settings.gain)}
      {input('lacunarity', settings.lacunarity)}
      {input('frequency', settings.frequency)}
    </Container>
  )
}

export default WorldControls
