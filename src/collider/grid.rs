use bevy::{prelude::*};
use std::collections::HashMap;
use rayon::prelude::ParallelIterator;

#[derive(Clone)]
pub struct ColliderDesc {
    pub id: usize,
    pub position: Vec2,
}

pub struct Cell {
    data: Vec<ColliderDesc>,
}

impl Cell {
    pub fn get_colliders(&self) -> impl Iterator<Item = &ColliderDesc> {
        self.data.iter()
    }
}

#[derive(Resource)]
pub struct Grid {
    cells: HashMap<(isize, isize), Cell>,
    radius: f32,
    cell_size: f32,
}

impl Grid {

    pub fn get_cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.values()
    }

    pub fn get_cell(&self, position: Vec2) -> (isize, isize) {
        let x = (position.x / self.cell_size).floor() as isize;
        let y = (position.y / self.cell_size).floor() as isize;
        (x, y)
    }
    pub fn new(cell_size: f32, radius: f32) -> Self {
        Self {
            cells: HashMap::default(),
            cell_size,
            radius,
        }
    }
    
    pub fn add(&mut self, collider: ColliderDesc) {
        let (x, y) = self.get_cell(collider.position);
        let cell = self.cells.entry((x, y)).or_insert_with(|| Cell {
            data: vec![],
        });

        cell.data.push(collider);
    }

    fn get(&self, position: Vec2) -> Option<&Cell> {
        let (x, y) = self.get_cell(position);
        self.cells.get(&(x, y))
    }


    pub fn clear(&mut self) {
        self.cells.clear();
    }

    fn get_neighbours(&self, position: Vec2) -> Vec<&Cell> {
        let (x, y) = self.get_cell(position);
        let mut neighbours = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if let Some(cell) = self.cells.get(&(x + i, y + j)) {
                    neighbours.push(cell);
                }
            }
        }
        neighbours
    }

    pub fn get_collided(&self, position: Vec2) -> Vec<&ColliderDesc> {
        let mut collided = vec![];
        for cell in self.get_neighbours(position) {
            for collider in &cell.data {
                if (collider.position - position).length() < self.radius && collider.position != position{
                    collided.push(collider);
                }
            }
        }
        collided
    }
}