import { useEffect, useCallback } from 'react'
import { useRecoilState, useRecoilValue } from 'recoil'
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
      const boundingRect = canvas.getBoundingClientRect()

      const scaleX =
        canvas.width / Math.ceil(window.devicePixelRatio) / boundingRect.width
      const scaleY =
        canvas.height / Math.ceil(window.devicePixelRatio) / boundingRect.height

      const canvasLeft = (event.clientX - boundingRect.left) * scaleX
      const canvasTop = (event.clientY - boundingRect.top) * scaleY

      const x = Math.min(Math.floor(canvasLeft), width - 1)
      const y = Math.min(Math.floor(canvasTop), height - 1)

      universe.paint(x, y, painter.brushSize, painter.type)
    }

    window.addEventListener('mousedown', handleClick)

    return () => window.removeEventListener('mousedown', handleClick)
  }, [painter, universe, canvas])

  return onPaint
}
