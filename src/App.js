import React, { useEffect, useState } from 'react'
import reglBuilder from 'regl'
import frag from './glsl/lit-element.glsl'
import vert from './glsl/vertex.glsl'
/* eslint-disable import/no-webpack-loader-syntax */
import { startSky } from "./scripts/sky"
// import SimulationWorker from 'worker-loader!./simulation.worker.js'
import * as Comlink from 'comlink'

// let fragmentShader = require('./glsl/element.glsl')
// let vertexShader = require('./glsl/vertex.glsl')

// const webWorkers = [
//   new SimulationWorker(),
//   // new SimulationWorker(),
// ]
// let simulators = []

const start = ({ canvas, universe, memory }) => {
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

let t = 0
let sky;
try {
  sky = startSky(1920 / (1920 / 12) * 2);
} catch (e) {
  console.error(e);
  sky = {
    frame: () => {}
  };
}
const App = ({ universe, memory, canvas }) => {
  console.log({ universe, memory, canvas })
  const [type, setType] = useState(2)

  useEffect(() => {
    const render = start({ universe, memory, canvas })

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
      let skyTime = dayTime / 255;
      window.skyTime = skyTime;
      sky.frame(skyTime);

      requestAnimationFrame(loop)
    }

    loop()
    return () => {}
  }, [])

  useEffect(() => {
    const handleClick = (event) => {
      console.log(event)
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

      universe.paint(x, y, 20, type)
    }

    window.addEventListener('mousedown', handleClick)

    return () => window.removeEventListener('mousedown', handleClick)
  }, [type])

  return (
    <button
      onClick={() => setType(type == 2 ? 0 : 2)}
      style={{ position: 'absolute', right: 0, top: 0 }}
    >
      type {type}
    </button>
  )
}

export default App
