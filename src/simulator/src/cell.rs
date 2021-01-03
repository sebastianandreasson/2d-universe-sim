use crate::element::Element;
use crate::element::ParticleElement;
use crate::element::PixelElement;
use crate::utils::rand_dir;
use crate::Physics;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
  pub sun: u8,
  pub sparkle: u8,
  pub b: u8,
  pub a: u8,
}

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

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Force {
  pub value: u8,
  pub direction: i8,
}

impl Force {
  pub fn new() -> Force {
    DEFAULT_FORCE
  }

  pub fn splash_force() -> Force {
    Force {
      value: 2,
      direction: rand_dir(),
    }
  }
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
  pub element: Element,
  pub light: u8,
  pub alpha: u8,
  pub clock: u8,
  pub force: Force,
  pub velocity: u8,
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
  pub element: PixelElement,
  pub light: u8,
  pub tmp: u8,
  pub alpha: u8,
}

impl Cell {
  pub fn new(element: Element) -> Cell {
    Cell {
      element,
      light: 75,
      alpha: 1,
      clock: 0,
      force: Force::new(),
      velocity: 1,
    }
  }
  pub fn cell_for_element(element: Element, generation: u8) -> Cell {
    Cell {
      element,
      light: 75,
      alpha: 1,
      clock: 0,
      force: Force::new(),
      velocity: 0,
    }
  }

  pub fn update(&self, physics: Physics) {
    self.element.update(*self, physics);
  }
  pub fn blocked_light(&self) -> f32 {
    self.element.blocked_light()
  }
  pub fn overwrite(&mut self, element: Element) {
    self.element = element;
    if element == Element::Empty {
      self.light = 75;
      self.alpha = 1;
    }
  }

  pub fn display(&self) -> Pixel {
    Pixel {
      element: PixelElement::from_element(self.element),
      light: self.light,
      alpha: self.alpha,
      tmp: 0,
    }
  }
}

pub static DEFAULT_FORCE: Force = Force {
  value: 1,
  direction: 0,
};

pub static EMPTY_CELL: Cell = Cell {
  element: Element::Empty,
  light: 75,
  alpha: 1,
  clock: 0,
  force: DEFAULT_FORCE,
  velocity: 0,
};
