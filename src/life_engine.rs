extern crate rand;

use life_engine::rand::prelude::*;
#[allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Dead,
    Alive
}

pub struct LifeEngine {
    pub width: u16,
    pub height: u16,
    matrix: Vec<Cell>
    
}

impl LifeEngine {
    pub fn new(width: u16, height: u16) -> Self {
        let w = width as usize;
        let h = height as usize;

        LifeEngine {
            width, 
            height,
            matrix: vec![Cell::Dead; w*h]
        }
    }

    pub fn fill_random(&mut self, probability: f32) {
        let mut rng = thread_rng();
        
        for i in 0..self.width*self.height {
            let chance: f32 = rng.gen();
            self.matrix[i as usize] = if chance < probability {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }
    }

    pub fn at(&self, x: i16, y: i16) -> Cell {
        if x < 0 || y < 0 {
            return Cell::Dead;
        }

        let x = x as u16;
        let y = y as u16;

        let index = y  * self.width + x;
        
        if index >= self.width * self.height {
            return Cell::Dead;
        }
        self.matrix[index as usize].clone()
    }

    pub fn count_vicinity(&self, x: i16, y: i16) -> u8 {
        let mut true_map = [false; 8];

        true_map[0] = self.at(x - 1, y - 1) == Cell::Alive;
        true_map[1] = self.at(x, y - 1) == Cell::Alive;
        true_map[2] = self.at(x + 1, y - 1) == Cell::Alive;
        true_map[3] = self.at(x - 1, y) == Cell::Alive;
        true_map[4] = self.at(x + 1, y) == Cell::Alive;
        true_map[5] = self.at(x - 1, y + 1) == Cell::Alive;
        true_map[6] = self.at(x, y + 1) == Cell::Alive;
        true_map[7] = self.at(x + 1, y + 1) == Cell::Alive;

        true_map.iter().filter(|&x| *x).count() as u8
    }

    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                let cell = self.matrix[index].clone();
                if cell == Cell::Dead && self.count_vicinity(x as i16, y as i16) == 3 {
                    self.matrix[index] = Cell::Alive;
                } 
                if cell == Cell::Alive {
                    let vicinity = self.count_vicinity(x as i16, y as i16);
                    if vicinity != 2 && vicinity != 3 {
                        self.matrix[index] = Cell::Dead;
                    }
                }
            }
        }
    }

    pub fn dump_matrix(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                print!("{}", match self.matrix[index].clone() {
                    Cell::Alive => "X",
                    Cell::Dead => "-"
                });
            }
            println!();
        }
    }
}