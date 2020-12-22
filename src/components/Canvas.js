import React from 'react'
import styled from 'styled-components'

const Container = styled.div`
  width: 100%;

  display: flex;
  flex: 1 1 auto;
  width: 0;
`

const Canvas = () => {
  return (
    <Container>
      <canvas id="canvas"></canvas>
    </Container>
  )
}

export default Canvas
