#![feature(async_closure)]

extern crate bracket_noise;
extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod cell;
mod element;
mod physics;
mod settings;
// mod pool;
mod utils;

use crate::bracket_noise::prelude::*;
use crate::cell::Cell;
use crate::cell::Light;
use crate::cell::EMPTY_CELL;
use crate::physics::Physics;
use crate::settings::NoiseGenerator;
use crate::settings::UniverseSettings;
use crate::utils::get_pkg_js_uri;
use element::Element;
use wasm_bindgen::prelude::*;
use wasm_mt_pool::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(a: f64);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    generation: u8,
    time: u8,
    cells: Vec<Cell>,
    lights: Vec<Light>,
    settings: UniverseSettings,
    // pool: ThreadPool,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        for x in (0..self.width).rev() {
            for y in (0..self.height).rev() {
                let cell = self.get_cell(x, y);

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
    }

    pub fn calculate_light(&mut self) {
        let time = ((self.time as f32) / 255.) * std::f32::consts::PI * 2.;

        // let time: f32 = 0.1;
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
            let blocked_light =
                (sunlight) * (1.0 - cell.blocked_light()) * ((r / ray_length) as f32) * 5.;
            sunlight = (sunlight) * cell.blocked_light();

            // let brx = rx - (dy * 0.) as i32;
            // let mut brx = rx + rand_dir() * 2;
            // let mut bry = ry + rand_dir() * 2;
            // if brx < 0 || brx > self.width - 1 {
            //     brx = rx;
            // }
            // if bry < 0 || bry > self.height - 1 {
            //     bry = ry;
            // }
            // let bounce_idx = self.get_index(brx, bry);
            // self.lights[bounce_idx].sparkle += blocked_light as u8;

            self.lights[idx].sun = sunlight as u8;
            self.lights[idx].b = self.lights[idx].b.saturating_sub(2);
            // self.lights[idx].sparkle = self.lights[idx].sparkle.saturating_sub(4);
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
    pub fn lights(&self) -> *const Light {
        self.lights.as_ptr()
    }

    pub fn paint(&mut self, x: i32, y: i32, size: i32, element: Element) {
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
                    self.cells[i] = Cell {
                        element: element,
                        velocity: 0,
                        alpha: 1,
                        clock: self.generation,
                    }
                }
            }
        }
    }

    pub fn set_time(&mut self, t: u8) {
        self.time = t;
    }
    pub fn inc_time(&mut self) {
        self.time = self.time.wrapping_add(1);
    }

    pub fn new(width: i32, height: i32, settings: UniverseSettings, generation: u8) -> Universe {
        let mut cells: Vec<Cell> = Vec::new();
        for y in 0..height {
            for x in 0..width {
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
        // let pkg_js_uri = get_pkg_js_uri();
        // let pool = ThreadPool::new(2, &pkg_js_uri).and_init().await.unwrap();

        Universe {
            width,
            height,
            generation,
            cells,
            lights,
            time: 0,
            settings,
            // pool,
        }
    }

    pub fn regenerate(&mut self, settings: UniverseSettings) {
        let noise = NoiseGenerator::new(settings).noise;

        for y in 0..self.height {
            for x in 0..self.width {
                let value = noise.get_noise(
                    (x as f32) / self.width as f32,
                    10.0 + (y as f32) / self.height as f32,
                );
                let i = self.get_index(x, y);
                if value < 0.025 {
                    self.cells[i].overwrite(Element::Ground);
                } else {
                    self.cells[i].overwrite(Element::Empty);
                }
            }
        }
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

    fn update_cell(cell: Cell, physics: Physics) {
        if cell.clock == physics.universe.generation {
            return;
        }

        cell.update(physics);
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
