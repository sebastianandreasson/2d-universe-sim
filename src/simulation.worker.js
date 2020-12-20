import * as Comlink from 'comlink'

let universe
let memory
let wasm
let index
const worker = {
  init: async (payload) => {
    import('./simulator/build').then(async (_wasm) => {
      console.log('worker.init', payload.universe.ptr)
      console.log('wasm', _wasm)
      memory = _wasm.wasm_memory()
      wasm = _wasm
      console.log('mry', memory)
      universe = memory.buffer[payload.universe.ptr]
      console.log('uni', universe)
    })
  },
  tick: () => wasm.tickUniverse(universe)
}


Comlink.expose(worker)