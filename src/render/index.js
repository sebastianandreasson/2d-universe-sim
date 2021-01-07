import reglBuilder from 'regl'
import startSky from './sky'
import frag from '../glsl/lit-element.glsl'
import vert from '../glsl/vertex.glsl'

let t = 0
export const start = ({ canvas, universe, memory }) => {
  const regl = reglBuilder({
    canvas,
    attributes: { preserveDrawingBuffer: false },
  })

  const width = universe.width()
  const height = universe.height()

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
      dataTexture: () =>
        regl.texture({
          width,
          height,
          data: new Uint8Array(
            memory.buffer,
            universe.pixels(),
            width * height * 4
          ),
        }),
      lightTexture: () =>
        regl.texture({
          width,
          height,
          data: new Uint8Array(
            memory.buffer,
            universe.lights(),
            width * height * 4
          ),
        }),
      particleTexture: () =>
        regl.texture({
          width,
          height,
          data: new Uint8Array(
            memory.buffer,
            universe.particles(),
            width * height * 4
          ),
        }),
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
      dpi: window.devicePixelRatio * 2,
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

export const renderLoop = ({ universe, memory, canvas, simulationOn }) => {
  let sky
  try {
    sky = startSky(universe.width())
  } catch (e) {
    console.error(e)
    sky = {
      frame: () => {},
    }
  }

  const render = start({ universe, canvas, memory })

  const loop = () => {
    if (simulationOn) universe.tick()
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
