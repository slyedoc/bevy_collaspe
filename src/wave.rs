use std::{process::Output, marker::PhantomData, fmt::Debug};

use bevy::prelude::*;
use rand::Rng;

pub trait Collapsible<T> {
    fn allowed_neighbors(&self) -> Vec<T>;
    fn values() -> Vec<T>;
}

enum Cell<T> {
    Possable(Vec<T>),
    Fixed(T),
}

pub struct Wave<T : Collapsible<T>> {
    size_x: usize,
    size_y: usize,
    cell_size: f32,
    grid: Vec<Vec<Cell<T>>>,
    //_phantom: PhantomData<T>,
}

impl<T : Collapsible<T> + Clone + Copy + Debug + Eq> Wave<T> {
    pub fn new(x: usize, y: usize, cell_size: f32) -> Self {
        let mut w = Wave {
            size_x: x,
            size_y: y,
            cell_size,
            grid: Vec::new(),
            //_phantom: PhantomData,
        };

        for x in 0..w.size_x {
            w.grid.push(Vec::new());
            for y in 0..w.size_y {
                w.grid[x].push(Cell::Possable(T::values()));
            }
        }

        w
    }

    pub fn seed_random(&mut self) {
        let mut rng= rand::thread_rng();
        let x = rng.gen_range(0..self.size_x);
        let y = rng.gen_range(0..self.size_y);
        self.collaspe_cell(x, y);
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: T) {
        // find neighbors and remove options that are invalid
        // check that the value is allowed
        match &self.grid[x][y] {
            Cell::Possable(values) => {
                if values.contains(&value) {
                    self.grid[x][y] = Cell::Fixed(value);
                }
                else {
                    panic!("cell value not allowed");
                }
            },
            Cell::Fixed(current) => {
                panic!("cell already set");
            },
        }
    }

    pub fn collaspe_cell(&mut self, x: usize, y: usize) {
        // find neighbors and remove options that are invalid
        // check that the value is allowed
        match &self.grid[x][y] {
            Cell::Possable(values) => {
                match values.len() {
                    0 => panic!("no values left"),
                    1 => {
                        self.grid[x][y] = Cell::Fixed(values[0]);
                    },
                    _ => {
                        let mut rng= rand::thread_rng();
                        let winner = values[rng.gen_range(0..values.len())];
                        self.grid[x][y] = Cell::Fixed(winner);
                    },
                }
            },
            Cell::Fixed(current) => {
                panic!("cell already collapsed");
            },
        }
    }

    pub fn print(self) {
        for row in self.grid.iter() {
            for col in row.iter() {
                match col {
                    Cell::Possable(possable) => {
                        print!("{:?}-", possable);
                    },
                    Cell::Fixed(fixed) => {
                        print!("|{:?}|-", fixed);
                    },
                }
            }
            println!(" ");
        }
    }
}

