use crate::element::Element;
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
pub struct Cell {
  pub element: Element,
  pub velocity: u8,
  pub alpha: u8,
  pub clock: u8,
}

impl Cell {
  pub fn new(element: Element) -> Cell {
    Cell {
      element: element,
      velocity: 75,
      alpha: 1,
      clock: 0,
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
  }
}

pub static EMPTY_CELL: Cell = Cell {
  element: Element::Empty,
  velocity: 0,
  alpha: 0,
  clock: 0,
};
