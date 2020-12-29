import { atom } from 'recoil'
import { ELEMENTS } from './constants'

export const universeState = atom({
  key: 'universe',
  default: null,
})

export const universeSettingsState = atom({
  key: 'universe-settings',
  default: {
    width: Math.floor(1920 / 15),
    height: Math.floor(1080 / 15),
    seed: 1,
    octaves: 4,
    gain: 0.5,
    lacunarity: 2.5,
    frequency: 1.0,
  },
})

export const positionState = atom({
  key: 'position',
  default: {
    x: 0,
    y: 0,
  },
})

export const wasmState = atom({
  key: 'wasm',
  default: null,
})

export const canvasState = atom({
  key: 'canvas',
  default: null,
})

export const painterState = atom({
  key: 'painter',
  default: {
    type: ELEMENTS.Water,
    brushSize: 10,
  },
})

export const simulationOnState = atom({
  key: 'simulation-switch',
  default: true,
})
