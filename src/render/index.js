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
  let cell_pointer = universe.pixels()
  let light_pointer = universe.lights()
  let particle_pointer = universe.lights()
  let cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4)
  let lights = new Uint8Array(memory.buffer, light_pointer, width * height * 4)
  let particles = new Uint8Array(
    memory.buffer,
    particle_pointer,
    width * height * 4
  )
  const dataTexture = regl.texture({ width, height, data: cells })
  const lightTexture = regl.texture({ width, height, data: lights })
  const particleTexture = regl.texture({ width, height, data: particles })

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
        cell_pointer = universe.pixels()
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
      particleTexture: () => {
        particle_pointer = universe.particles()
        particles = new Uint8Array(
          memory.buffer,
          particle_pointer,
          width * height * 4
        )
        return particleTexture({ width, height, data: particles })
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
