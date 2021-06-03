mod utils;

use js_sys::*;
use wasm_bindgen::prelude::*;

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

    // algorithm is taken from
    // https://www.geeksforgeeks.org/rotate-a-matrix-by-90-degree-in-clockwise-direction-without-using-any-extra-space/
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

    #[allow(non_snake_case)]
    pub fn toString(&self) -> String {
        let mut result = String::from("");

        for i in 0..self.height {
            let mut row = String::from("");
            for j in 0..self.width {
                row.push_str(&self[(i, j)].to_string());
            }
            row.push_str("\n");
            result.push_str(&row);
        }

        result
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
pub fn create_playfield() -> Matrix {
    Matrix {
        width: 10,
        height: 10,
        matrix: vec![0; 400]
    }
}

#[wasm_bindgen]
pub fn put_block(playfield: &mut Matrix, block: &Matrix, x: usize, y: usize) {
    log(&format!("x: {}, y: {}, block.w: {}, block.h: {}", x, y, block.width, block.height));
    assert!(x + block.width < playfield.width);
    assert!(y + block.height < playfield.height);

    for ly in 0..block.height {
        for lx in 0..block.width {
            playfield[(lx + x, ly + y)] = block[(lx, ly)];
        }
    }
}

#[wasm_bindgen]
pub fn create_i_block() -> Matrix {
    Matrix {
        width: 4,
        height: 4,
        matrix: vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    }
}

#[wasm_bindgen]
pub fn create_j_block() -> Matrix {
    Matrix {
        width: 3,
        height: 3,
        matrix: vec![1, 0, 0, 1, 1, 1, 0, 0, 0],
    }
}

#[wasm_bindgen]
pub fn create_l_block() -> Matrix {
    Matrix {
        width: 3,
        height: 3,
        matrix: vec![0, 0, 1, 1, 1, 1, 0, 0, 0],
    }
}

#[wasm_bindgen]
pub fn create_o_block() -> Matrix {
    Matrix {
        width: 2,
        height: 2,
        matrix: vec![1, 1, 1, 1],
    }
}

#[wasm_bindgen]
pub fn create_s_block() -> Matrix {
    Matrix {
        width: 3,
        height: 3,
        matrix: vec![0, 1, 1, 1, 1, 0, 0, 0, 0],
    }
}

#[wasm_bindgen]
pub fn create_t_block() -> Matrix {
    Matrix {
        width: 3,
        height: 3,
        matrix: vec![0, 1, 0, 1, 1, 1, 0, 0, 0],
    }
}

#[wasm_bindgen]
pub fn create_z_block() -> Matrix {
    Matrix {
        width: 3,
        height: 3,
        matrix: vec![1, 1, 0, 0, 1, 1, 0, 0, 0],
    }
}

#[wasm_bindgen]
pub enum BlockType {
    I = 0,
    J = 1,
    L = 2,
    O = 3,
    S = 4,
    T = 5,
    Z = 6,
}

#[wasm_bindgen]
pub fn create_block(block_type: BlockType) -> Matrix {
    match block_type {
        BlockType::I => create_i_block(),
        BlockType::J => create_j_block(),
        BlockType::L => create_l_block(),
        BlockType::O => create_o_block(),
        BlockType::S => create_s_block(),
        BlockType::T => create_t_block(),
        BlockType::Z => create_z_block(),
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

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
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
