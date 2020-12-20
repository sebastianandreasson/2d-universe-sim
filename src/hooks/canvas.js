import { useEffect } from 'react'
import { useRecoilState } from 'recoil'
import { canvasState } from '../state'

export const useCanvas = () => {
  const [canvas, setCanvas] = useRecoilState(canvasState)

  const onResize = useEffect(() => {}, [canvas])

  useEffect(() => {
    setCanvas(document.getElementById('canvas'))
  }, [])

  return canvas
}
