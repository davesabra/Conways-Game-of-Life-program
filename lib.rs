#![allow(unused_variables)]
mod utils;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use std::fmt;
extern crate fixedbitset; // to represent cells instead of Vec<Cell>
use fixedbitset::FixedBitSet;

#[wasm_bindgen]
//#[repr(u8)]                     //<-- 1 byte per cell version
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead =  0,
    Alive = 1,
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet, // type to represent cells instead of Vec<Cell>
}

impl Universe {
    
    fn get_index(&self, row: u32, column: u32) -> usize {                //<-- access the cell at a given row and column, index into cell's vector
        (row * self.width + column) as usize
    }


    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {         //<-- count how many of cells's neihgbours are alive
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {       //<-- uses deltas
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;       // <-- and modulo to avoid special casing the b.c. s with ifs
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match (cell, live_neighbors) {
                   (true, x) if x < 2 => false,
                   (true, 2) | (true, 3) => true,
                   (true, x) if x > 3 => false,
                   (false, 3) => true,
                   (otherwise, _) => otherwise
                });
                /*
                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                */
                //next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
    /*
    // THIS IS PERTINENT TO CELL PER BIT OPTIM
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
    */
}

impl fmt::Display for Universe {                                             //<-- basic text render for human readable output
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                //let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    // ...
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    
    /* //not using Cell anymore
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    */

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()  // view the bitset as a slice of u32 blocks
                                        // returning a raw ptr to the slice's buffer
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        
        // modified init algorithm for the bit-optimised version
        //for i in 0..size {
        //cells.set(i, i % 2 == 0 || i % 7 == 0);
        //}

        //  possible random i.c's for bit-optimised version
        for i in 0..size {
        cells.set(i,
            {let i = js_sys::Math::random();
                if i < 0.7 {
                   true                            //Cell::Alive
                } else {
                   false                            //Cell::Dead
                } 
            }    
        )    
        }
        
        /*
        let cells = (0..width * height)
            .map(|i| { 
                 let i = js_sys::Math::random();
                 if i < 0.2 {                           // <--closure to set I.C.s
                 //if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        */    

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {                  // <-- maybe impl bitmap later
        self.to_string()
    }
}
