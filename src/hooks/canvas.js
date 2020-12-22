import { useEffect } from 'react'
import { useRecoilState } from 'recoil'
import { canvasState } from '../state'

export const useCanvas = () => {
  const [canvas, setCanvas] = useRecoilState(canvasState)

  useEffect(() => {
    if (!canvas) return

    const resize = () => {
      let screen_width = window.innerWidth
      let screen_height = window.innerHeight

      canvas.style = `width: ${screen_width}px; height: ${screen_height}px;`
      console.log(canvas.style)
    }

    resize()
    window.addEventListener('resize', resize)

    return () => window.removeEventListener('resize', resize)
  }, [canvas])

  useEffect(() => {
    setCanvas(document.getElementById('canvas'))
  }, [])

  return canvas
}
