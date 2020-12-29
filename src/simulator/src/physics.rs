use crate::Cell;
use crate::Element;
use crate::Universe;

pub struct Physics<'a> {
  pub x: i32,
  pub y: i32,
  pub universe: &'a mut Universe,
}

impl<'a> Physics<'a> {
  pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
    let nx = self.x + dx;
    let ny = self.y + dy;
    if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
      return Cell::cell_for_element(Element::Rock, self.universe.generation);
    }
    return self.universe.get_cell(nx, ny);
  }

  pub fn set(&mut self, dx: i32, dy: i32, cell: Cell) {
    let nx = self.x + dx;
    let ny = self.y + dy;

    if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
      return;
    }
    let i = self.universe.get_index(nx, ny);
    self.universe.cells[i] = cell;
    self.universe.cells[i].velocity = 1;
    self.universe.cells[i].clock = self.universe.generation.wrapping_add(1);
  }

  pub fn set_and_inc_velocity(&mut self, dx: i32, dy: i32, cell: Cell) {
    let nx = self.x + dx;
    let ny = self.y + dy;

    if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
      return;
    }
    let i = self.universe.get_index(nx, ny);
    self.universe.cells[i] = cell;
    self.universe.cells[i].clock = self.universe.generation.wrapping_add(1);
    self.universe.cells[i].velocity += 1;
  }
}
