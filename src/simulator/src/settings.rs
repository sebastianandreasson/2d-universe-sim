use crate::bracket_noise::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniverseSettings {
  pub seed: i32,
  pub octaves: i32,
  pub gain: f32,
  pub lacunarity: f32,
  pub frequency: f32,
  pub water_level: f32,
}

#[wasm_bindgen]
impl UniverseSettings {
  pub fn new(
    seed: i32,
    octaves: i32,
    gain: f32,
    lacunarity: f32,
    frequency: f32,
    water_level: f32,
  ) -> UniverseSettings {
    UniverseSettings {
      seed,
      octaves,
      gain,
      lacunarity,
      frequency,
      water_level,
    }
  }
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[wasm_bindgen]
impl Position {
  pub fn new(x: i32, y: i32) -> Position {
    Position { x, y }
  }

  pub fn set(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }
}

pub struct NoiseGenerator {
  pub noise: FastNoise,
}

impl NoiseGenerator {
  pub fn new(settings: UniverseSettings) -> NoiseGenerator {
    let mut noise = FastNoise::seeded(settings.seed as u64);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(settings.octaves);
    noise.set_fractal_gain(settings.gain);
    noise.set_fractal_lacunarity(settings.lacunarity);
    noise.set_frequency(settings.frequency);
    NoiseGenerator { noise }
  }
}
