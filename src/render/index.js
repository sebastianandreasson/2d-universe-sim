import reglBuilder from 'regl'
import startSky from './sky'
import frag from '../glsl/lit-element.glsl'
import vert from '../glsl/vertex.glsl'

let t = 0
export const start = ({ canvas, universe, memory }) => {
  console.log('regl', canvas)
  const regl = reglBuilder({
    canvas,
    attributes: { preserveDrawingBuffer: false },
  })

  const width = universe.width()
  const height = universe.height()
  let cell_pointer = universe.cells()
  let light_pointer = universe.lights()
  let cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4)
  let lights = new Uint8Array(memory.buffer, light_pointer, width * height * 4)
  const dataTexture = regl.texture({ width, height, data: cells })
  const lightTexture = regl.texture({ width, height, data: lights })

  const draw = regl({
    blend: {
      enable: true,
      func: {
        srcRGB: 'src alpha',
        srcAlpha: 1,
        dstRGB: 'one minus src alpha',
        dstAlpha: 1,
      },
      equation: {
        rgb: 'add',
        alpha: 'add',
      },
      color: [0, 0, 0, 0],
    },
    frag,
    vert,
    uniforms: {
      t: ({ tick }) => tick,
      skyTime: () => (window.t ? window.t : 0),
      dataTexture: () => {
        cell_pointer = universe.cells()
        cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4)
        return dataTexture({ width, height, data: cells })
      },
      lightTexture: () => {
        light_pointer = universe.lights()

        lights = new Uint8Array(
          memory.buffer,
          light_pointer,
          width * height * 4
        )

        return lightTexture({ width, height, data: lights })
      },
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
      dpi: 4,
      // dpi: window.devicePixelRatio * 2,
    },
    attributes: {
      position: [
        [-1, 4],
        [-1, -1],
        [4, -1],
      ],
    },
    count: 3,
  })

  return () => {
    regl.poll()
    draw()
  }
}

export const renderLoop = ({ universe, memory, canvas }) => {
  let sky
  try {
    sky = startSky((1920 / (1920 / 12)) * 2)
  } catch (e) {
    console.error(e)
    sky = {
      frame: () => {},
    }
  }

  const render = start({ universe, canvas, memory })

  const loop = () => {
    universe.tick()
    var dayTime = (t / 50) % 255
    t += 1

    if (dayTime > 70 && dayTime < 170) {
      t += 10
    }
    window.t = t
    universe.set_time(dayTime)
    render()
    let skyTime = dayTime / 255
    window.skyTime = skyTime
    sky.frame(skyTime)

    requestAnimationFrame(loop)
  }
  loop()
}
