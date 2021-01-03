use crate::cell::Force;
use crate::cell::DEFAULT_FORCE;
use crate::element::ParticleElement;
use crate::element::PixelElement;
use crate::Physics;
use crate::Pixel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
  pub element: ParticleElement,
  pub force: Force,
  pub clock: u8,
  pub alpha: u8,
}

impl Particle {
  pub fn new(element: ParticleElement, generation: u8) -> Particle {
    Particle {
      element,
      force: Force::splash_force(),
      clock: generation,
      alpha: 100,
    }
  }
  pub fn update(&self, physics: Physics) {
    self.element.update(*self, physics);
  }
  pub fn display(&self) -> Pixel {
    Pixel {
      element: PixelElement::from_particle_element(self.element),
      light: 100,
      tmp: 0,
      alpha: self.alpha,
    }
  }
}

pub static EMPTY_PARTICLE: Particle = Particle {
  element: ParticleElement::Empty,
  force: DEFAULT_FORCE,
  clock: 0,
  alpha: 0,
};
