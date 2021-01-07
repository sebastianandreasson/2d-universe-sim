#![feature(async_closure)]

extern crate bracket_noise;
extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod cell;
mod element;
mod particle;
mod physics;
mod settings;
mod utils;

use crate::cell::Cell;
use crate::cell::Light;
use crate::cell::Pixel;
use crate::cell::EMPTY_CELL;
use crate::particle::Particle;
use crate::particle::EMPTY_PARTICLE;
use crate::physics::Physics;
use crate::settings::NoiseGenerator;
use crate::settings::Position;
use crate::settings::UniverseSettings;
use crate::utils::rand_to;
use element::Element;
use element::ParticleElement;
use element::PixelElement;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    generation: u8,
    time: u8,
    cells: Vec<Cell>,
    lights: Vec<Light>,
    particles: Vec<Particle>,
    sprites: Vec<Pixel>, // pool: ThreadPool,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for x in (0..self.width).rev() {
            for y in (0..self.height).rev() {
                if self.generation % 2 == 0 {
                    continue;
                }
                let cell = self.get_cell(x, y);
                let particle = self.get_particle(x, y);
                Universe::update_particle(
                    particle,
                    Physics {
                        universe: self,
                        x,
                        y,
                    },
                );
                Universe::update_cell(
                    cell,
                    Physics {
                        universe: self,
                        x,
                        y,
                    },
                );
            }
        }
        self.calculate_light();
        self.generation = self.generation.wrapping_add(1);
    }

    pub fn calculate_light(&mut self) {
        let time = ((self.time as f32) / 255.) * std::f32::consts::PI * 2.;

        let mut dx = time.sin();
        let mut dy = time.cos();

        let mut brightness = 255.0;
        if dy < 0.5 {
            brightness = 30.0 + (225.0 * (dy / 0.5)) as f32;
        }
        if dy < -0.1 {
            dx = 0.0;
            dy = 1.0;
        }

        let start_y = if dy > 0. { 0 } else { self.height - 1 };
        for start_x in 0..self.width {
            self.cast_ray(brightness, start_x, start_y, dx, dy);
        }

        let start_x = if dx > 0. { 0 } else { self.width - 1 };
        for start_y in 0..self.height {
            self.cast_ray(brightness, start_x, start_y, dx, dy);
        }
    }
    pub fn cast_ray(&mut self, brightness: f32, x: i32, y: i32, dx: f32, dy: f32) {
        let ray_length =
            (((self.width * self.width) + (self.height * self.height)) as f32).sqrt() as i32;

        let mut sunlight: f32 = brightness;
        for r in 0..ray_length {
            let rx = (r as f32 * dx) as i32 + x;
            let ry = (r as f32 * dy) as i32 + y;

            let idx = self.get_index(rx, ry);
            if rx < 0
                || ry < 0
                || rx >= self.width
                || ry >= self.height
                || idx > self.get_max_index()
            {
                break;
            }
            let cell = self.get_cell(rx, ry);
            sunlight = (sunlight) * cell.blocked_light();

            self.lights[idx].sun = sunlight as u8;
            self.lights[idx].b = self.lights[idx].b.saturating_sub(2);
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn pixels(&self) -> *const Pixel {
        return self
            .cells
            .iter()
            .map(|&c| c.display())
            .collect::<Vec<Pixel>>()
            .as_ptr();
    }
    pub fn lights(&self) -> *const Light {
        self.lights.as_ptr()
    }
    pub fn sprites(&self) -> *const Pixel {
        self.sprites.as_ptr()
    }
    pub fn particles(&self) -> *const Pixel {
        return self
            .particles
            .iter()
            .map(|&p| p.display())
            .collect::<Vec<Pixel>>()
            .as_ptr();
    }

    pub fn paint(&mut self, x: i32, y: i32, size: i32, element: Element) {
        if size == 1 {
            let i = self.get_index(x, y);
            let cell = self.get_cell(x, y);
            if cell.element == Element::Empty || element == Element::Empty {
                self.cells[i] = Cell::cell_for_element(element, self.generation);
            }
            return;
        }

        let radius = size / 2;
        for dx in -radius..radius + 1 {
            for dy in -radius..radius + 1 {
                if dx * dx + dy * dy > (radius * radius) - 1 {
                    continue;
                };
                let px = x + dx;
                let py = y + dy;

                let i = self.get_index(px, py);

                if px < 0 || px > self.width - 1 || py < 0 || py > self.height - 1 {
                    continue;
                }
                let cell = self.get_cell(px, py);
                if cell.element == Element::Empty || element == Element::Empty {
                    self.cells[i] = Cell::cell_for_element(element, self.generation);
                }
            }
        }
    }

    pub fn paint_particle(&mut self, x: i32, y: i32) {
        let i = self.get_index(x, y);
        let cell = self.get_cell(x, y);
        if cell.element == Element::Empty {
            self.particles[i] = Particle::new(ParticleElement::Foam, self.generation);
        }
    }

    pub fn set_time(&mut self, t: u8) {
        self.time = t;
    }
    pub fn inc_time(&mut self) {
        self.time = self.time.wrapping_add(1);
    }

    pub fn new(width: i32, height: i32, generation: u8) -> Universe {
        let mut cells: Vec<Cell> = Vec::new();
        for _ in 0..height {
            for _ in 0..width {
                cells.push(EMPTY_CELL);
            }
        }

        let lights: Vec<Light> = (0..(width * height))
            .map(|_i| Light {
                sun: 0,
                sparkle: 0,
                b: 0,
                a: 0,
            })
            .collect();
        let particles: Vec<Particle> = (0..(width * height)).map(|_i| EMPTY_PARTICLE).collect();
        let sprites: Vec<Pixel> = (0..(width * height))
            .map(|_i| Pixel {
                alpha: 0,
                element: PixelElement::Empty,
                light: 0,
                tmp: 0,
            })
            .collect();

        Universe {
            width,
            height,
            generation,
            cells,
            lights,
            particles,
            sprites,
            time: 0,
        }
    }

    pub fn regenerate(&mut self, settings: UniverseSettings, position: Position) {
        self.generation = 0;
        let noise = NoiseGenerator::new(settings).noise;
        let mut prev_value: f32 = 0.0;
        let mut cells: Vec<Cell> = Vec::new();
        for _ in 0..self.height {
            for _ in 0..self.width {
                cells.push(EMPTY_CELL);
            }
        }
        self.cells = cells;

        for x in 0..self.width {
            for y in 0..self.height {
                let i = self.get_index(x, y);

                let x_off = ((x as f32) + position.x as f32) / self.width as f32;
                let y_off = (10.0 + (y as f32) + position.y as f32) / self.height as f32;
                let value = noise.get_noise(x_off, y_off);
                // log_f64(value as f64);

                if value < 0.02 {
                    if prev_value > 0.001 {
                        self.cells[i].overwrite(Element::Grass);
                        self.cells[i].light = 10;
                    } else if prev_value > -0.2 {
                        self.cells[i].overwrite(Element::Dirt);
                        self.cells[i].light = 10;
                    } else {
                        self.cells[i].overwrite(Element::Rock);
                    }
                } else {
                    if y as f32 >= (self.height as f32 * 0.8) {
                        self.cells[i].overwrite(Element::Water);
                    } else {
                        self.cells[i].overwrite(Element::Empty);
                    }
                }

                prev_value = value;
            }
        }
        self.lights = (0..(self.width * self.height))
            .map(|_i| Light {
                sun: 0,
                sparkle: 0,
                b: 0,
                a: 0,
            })
            .collect();
        self.calculate_light();
        self.particles = (0..(self.width * self.height))
            .map(|_i| EMPTY_PARTICLE)
            .collect();
    }
    pub fn debug() -> i8 {
        return rand_to(5) as i8;
    }
}

impl Universe {
    fn get_index(&self, x: i32, y: i32) -> usize {
        (x + (y * self.width)) as usize
    }
    fn get_max_index(&self) -> usize {
        let m_w = (self.width - 1) as usize;
        let m_h = self.height - 1;
        return (m_w + (m_h * self.width) as usize) as usize;
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        let i = self.get_index(x, y);
        return self.cells[i];
    }
    fn get_particle(&self, x: i32, y: i32) -> Particle {
        let i = self.get_index(x, y);
        return self.particles[i];
    }

    fn update_cell(cell: Cell, physics: Physics) {
        if cell.clock == physics.universe.generation {
            return;
        }

        cell.update(physics);
    }

    fn update_particle(particle: Particle, physics: Physics) {
        if particle.element == ParticleElement::Empty
            || particle.clock >= physics.universe.generation
        {
            return;
        }

        particle.update(physics);
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
