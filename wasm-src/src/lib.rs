mod utils;

use wasm_bindgen::prelude::*;
use js_sys::*;

use std::ops::{Index, IndexMut};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    matrix: Vec<u8>,
}

type Coord = (usize, usize);

#[wasm_bindgen]
impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            width: width,
            height: height,
            matrix: vec![0; width * height],
        }
    }

    #[wasm_bindgen(getter)]
    pub fn to_uint8array(&self) -> Uint8Array {
        return Uint8Array::from(&self.matrix[..]);
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self[(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self[(x, y)] = value;
    }

    pub fn rotate_right(&mut self) {
        assert_eq!(self.width, self.height);
        let n = self.height;

        for i in 0..(n / 2) {
            for j in i..(n - i - 1) {
                let temp = self[(i, j)];
                self[(i, j)] = self[(n - 1 - j, i)];
                self[(n - 1 - j, i)] = self[(n - 1 - i, n - 1 - j)];
                self[(n - 1 - i, n - 1 - j)] = self[(j, n - 1 - i)];
                self[(j, n - 1 - i)] = temp;
            }
        }
    }
}

impl Index<Coord> for Matrix {
    type Output = u8;

    fn index(&self, index: Coord) -> &Self::Output {
        return &self.matrix[index.0 * self.height + index.1];
    }
}

impl IndexMut<Coord> for Matrix {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.matrix[index.0 * self.height + index.1]
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
