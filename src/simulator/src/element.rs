use super::utils::*;
use crate::particle::EMPTY_PARTICLE;
// use crate::log_u32;
use crate::particle::Particle;

use crate::cell::Cell;
use crate::Physics;
use crate::EMPTY_CELL;
use wasm_bindgen::prelude::*;
// use web_sys::console;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum PixelElement {
  Empty = 0,
  Water = 1,
  Rock = 10,
  Dirt = 11,
  Grass = 12,
}

impl PixelElement {
  pub fn from_element(element: Element) -> PixelElement {
    match element {
      Element::Empty => PixelElement::Empty,
      Element::Rock => PixelElement::Rock,
      Element::Dirt => PixelElement::Dirt,
      Element::Grass => PixelElement::Grass,
      Element::Water => PixelElement::Water,
    }
  }
  pub fn from_particle_element(element: ParticleElement) -> PixelElement {
    match element {
      ParticleElement::Empty => PixelElement::Empty,
      ParticleElement::Foam => PixelElement::Water,
    }
  }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum Element {
  Empty = 0,
  Water = 1,
  Rock = 10,
  Dirt = 11,
  Grass = 12,
}

impl Element {
  pub fn update(&self, cell: Cell, physics: Physics) {
    match self {
      Element::Empty => {}
      Element::Rock => {}
      Element::Dirt => {}
      Element::Grass => {}
      Element::Water => update_liquid(cell, physics),
    }
  }
  pub fn blocked_light(&self) -> f32 {
    (100.0
      - match self {
        Element::Empty => 0.0,
        Element::Water => 1.0,
        Element::Rock => 40.0,
        Element::Grass => 20.0,
        Element::Dirt => 10.0,
      })
      / 100.0
  }
}

// pub fn test(mut cell: Cell, mut phys: Physics) -> Cell {
//   EMPTY_CELL
// }

pub fn update_liquid(cell: Cell, mut phys: Physics) {
  let mut dy: i8 = 0;
  if cell.velocity > 0 {
    for v in 1..cell.velocity {
      dy = v as i8;
      if phys.get(0, v as i8).element != Element::Empty {
        dy = (v - 1) as i8;
        break;
      }
    }
  }

  let dx = rand_dir();
  let dx0 = phys.get(dx, dy);
  let dx1 = phys.get(dx, dy + 1);

  if phys.get(0, dy + 1).element == Element::Empty {
    phys.set(0, 0, EMPTY_CELL);
    phys.set_and_inc_velocity(0, dy + 1, cell);
  } else if dx1.element == Element::Empty {
    phys.set(0, 0, dx1);
    phys.set(dx, dy + 1, cell);
  } else if phys.get(-dx, dy + 1).element == Element::Empty {
    phys.set(0, 0, EMPTY_CELL);
    phys.set(-dx, dy + 1, cell);
  } else if dx0.element == Element::Empty {
    phys.set(0, 0, dx0);
    phys.set(dx, dy, cell);
  } else if phys.get(-dx, 0).element == Element::Empty {
    phys.set(0, 0, EMPTY_CELL);
    phys.set(-dx, dy, cell);
  }
  if dx1.element == Element::Water && cell.velocity > 5 {
    phys.set(0, 0, EMPTY_CELL);
    phys.set_particle(dx, dy, Particle::new(ParticleElement::Foam, cell.clock));
  }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum ParticleElement {
  Empty = 0,
  Foam = 1,
}

impl ParticleElement {
  pub fn update(&self, particle: Particle, physics: Physics) {
    match self {
      ParticleElement::Empty => {}
      ParticleElement::Foam => update_particle(particle, physics),
    }
  }
}

pub fn update_particle(particle: Particle, mut phys: Physics) {
  let mut dir = particle.force.direction;

  if particle.force.value == 0 {
    phys.set_particle(0, 0, EMPTY_PARTICLE);
    phys.set(0, 0, Cell::new(Element::Water));
    return;
  }

  phys.set_particle(0, 0, EMPTY_PARTICLE);
  if phys.get(dir, -1).element > Element::Rock {
    dir = -dir;
  }
  phys.set_particle(dir, -1, particle);
}
