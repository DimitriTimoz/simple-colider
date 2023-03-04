use bevy::prelude::*;

use crate::body::Mass;

#[derive(Component)]
pub struct Gravity {
    pub center: Vec3,
    pub mass: f32,
}


impl Gravity {
    pub fn new(center: Vec3, mass: f32) -> Self {
        Self {
            center,
            mass,
        }
    }

    pub fn apply_gravity(&self, mass: Mass, position: Vec3) -> Vec3 {
        let direction = (self.center - position).normalize();
        direction * 9.81 
    }
    
}