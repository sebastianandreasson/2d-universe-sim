import { useEffect } from 'react'
import { useRecoilValue } from 'recoil'
import { canvasState, painterState, universeState } from '../state'

const clickInsideUI = (x, y) => {
  const width = window.innerWidth

  if (x >= width - 200) {
    return true
  }
  return false
}

export const usePainter = () => {
  const painter = useRecoilValue(painterState)
  const canvas = useRecoilValue(canvasState)
  const { universe } = useRecoilValue(universeState)

  const onPaint = useEffect(() => {
    const handleClick = (event) => {
      if (clickInsideUI(event.clientX, event.clientY)) return
      const width = universe.width()
      const height = universe.height()

      const scaleX = width / canvas.getBoundingClientRect().width
      const scaleY = height / canvas.getBoundingClientRect().height
      const x = Math.floor(event.clientX * scaleX)
      const y = Math.floor(event.clientY * scaleY)

      universe.paint(x, y, painter.brushSize, painter.type)
      // universe.paint_particle(x, y)
    }

    window.addEventListener('mousedown', handleClick)

    return () => window.removeEventListener('mousedown', handleClick)
  }, [painter, universe, canvas])

  return onPaint
}
