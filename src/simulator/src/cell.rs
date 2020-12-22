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
pub struct Force {
  pub value: u8,
  pub direction: u8,
}

impl Force {
  pub fn new() -> Force {
    Force {
      value: 0,
      direction: 0,
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
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
  pub element: Element,
  pub light: u8,
  pub alpha: u8,
  pub tmp: u8,
}

impl Cell {
  pub fn new(element: Element) -> Cell {
    Cell {
      element: element,
      light: 75,
      alpha: 1,
      clock: 0,
      force: Force::new(),
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
      element: self.element,
      light: self.light,
      alpha: self.alpha,
      tmp: 0,
    }
  }
}

pub static EMPTY_CELL: Cell = Cell {
  element: Element::Empty,
  light: 75,
  alpha: 1,
  clock: 0,
  force: Force {
    value: 0,
    direction: 0,
  },
};
