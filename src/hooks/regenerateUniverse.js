import { useEffect } from 'react'
import { useRecoilValue } from 'recoil'
import {
  universeSettingsState,
  universeState,
  positionState,
  wasmState,
} from '../state'

export const useRegenerateUniverse = () => {
  const wasm = useRecoilValue(wasmState)
  const { universe } = useRecoilValue(universeState)
  const settings = useRecoilValue(universeSettingsState)
  const position = useRecoilValue(positionState)

  useEffect(() => {
    if (!universe) return
    const pos = wasm.Position.new(position.x, position.y)
    const universeSettings = wasm.UniverseSettings.new(
      settings.seed,
      settings.octaves,
      settings.gain,
      settings.lacunarity,
      settings.frequency
    )
    universe.regenerate(universeSettings, pos)
  }, [wasm, universe, settings, position])
}
