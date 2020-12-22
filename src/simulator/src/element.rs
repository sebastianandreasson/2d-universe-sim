use super::utils::*;

use crate::cell::Cell;
use crate::Physics;
use crate::EMPTY_CELL;
use wasm_bindgen::prelude::*;
// use web_sys::console;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Element {
  Empty = 0,
  Ground = 1,
  Water = 2,
  Dirt = 3,
  Grass = 4,
}

impl Element {
  pub fn update(&self, cell: Cell, physics: Physics) {
    match self {
      Element::Empty => {}
      Element::Ground => {}
      Element::Dirt => {}
      Element::Grass => {}
      Element::Water => update_water(cell, physics),
    }
  }
  pub fn blocked_light(&self) -> f32 {
    (100.0
      - match self {
        Element::Empty => 0.0,
        Element::Water => 1.0,
        Element::Ground => 30.0,
        Element::Grass => 20.0,
        Element::Dirt => 10.0,
      })
      / 100.0
  }
}

pub fn update_water(mut cell: Cell, mut phys: Physics) {
  let dx = rand_dir();
  let dx1 = phys.get(dx, 1);
  let dx0 = phys.get(dx, 0);

  if phys.get(0, 1).element == Element::Empty {
    phys.set(0, 0, EMPTY_CELL);
    phys.set(0, 1, cell);
  } else if dx1.element == Element::Empty {
    phys.set(0, 0, dx1);
    phys.set(dx, 1, cell);
  } else if phys.get(-dx, 1).element == Element::Empty {
    phys.set(0, 0, EMPTY_CELL);
    phys.set(-dx, 1, cell);
  } else if dx0.element == Element::Empty {
    phys.set(0, 0, dx0);
    phys.set(dx, 0, cell);
  } else if phys.get(-dx, 0).element == Element::Empty {
    phys.set(0, 0, EMPTY_CELL);
    phys.set(-dx, 0, cell);
  }
  // cell.alpha = 1;
}
