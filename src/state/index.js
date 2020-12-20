import { atom, selector } from 'recoil'
import { ELEMENTS } from './constants'

export const universeState = atom({
  key: 'universe',
  default: null,
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
    brushSize: 20,
  },
})
