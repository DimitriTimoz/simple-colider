use std::ops::Add;

use bevy::prelude::*;

use crate::{gravity::Gravity, ball::Radius};

#[derive(Component, Clone, Copy)]
pub struct DynamicBody {
    pub mass: Mass,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

#[derive(Component, Clone, Copy)]
pub struct Mass(pub f32);

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec3);

#[derive(Component, Clone, Copy)]
pub struct Acceleration(Vec3);


impl Default for DynamicBody {
    fn default() -> Self {
        Self {
            mass: Mass(1.0),
            velocity: Velocity(Vec3::ZERO),
            acceleration: Acceleration(Vec3::ZERO),
        }
    }
}

impl DynamicBody {
    pub fn apply_force(&mut self, force: Vec3) {
        self.acceleration.0 = force / self.mass.0;
    }

    pub fn apply_impulse(&mut self, impulse: Vec3) {
        self.velocity.0 = impulse / self.mass.0;
    }

    fn apply_gravity(
        mut query_bodies: Query<(&mut DynamicBody, &Transform)>,
        query_gravities: Query<&Gravity>
    ) {
        for gravity in query_gravities.iter() {
            for (mut body, transfrom) in query_bodies.iter_mut() {
                let force = gravity.apply_gravity(body.mass, transfrom.translation);
                body.apply_force(force);
            }
        }
    }

    fn apply_physics(
        mut bodies: Query<(&mut Transform, &mut DynamicBody)>,
        time: Res<Time>,
    ) {
        for (mut transform, mut body) in bodies.iter_mut() {
            let acceleration = body.acceleration.0;
            body.velocity.0 += acceleration * time.delta_seconds();
            transform.translation += body.velocity.0 * time.delta_seconds();
        }
    }

    fn fix_colisions( 
        mut query: Query<(&mut Transform, &mut DynamicBody, &Radius)>,
        time: Res<Time>,
    ) {

        let mut objects = query.iter_mut().collect::<Vec<_>>();

        for i in 0..objects.len() {
            for j in 1..objects.len() {
               
                // Check if collision 
                let radius = objects[i].2.0 + objects[j].2.0;

                let distance = objects[i].0.translation.distance(objects[j].0.translation);
                if radius > distance && distance != 0.0{
                    let vel1 = objects[i].1.velocity.0;
                    let vel2 = objects[j].1.velocity.0;
                    objects[i].1.velocity.0 = vel2 * 0.99;
                    objects[j].1.velocity.0 = vel1 * 0.99;

                    // fix position
                    let to_fix = (radius - distance ) / 2.0;

                    let trans1 = objects[i].0.translation;
                    let trans2 = objects[j].0.translation;
                    objects[i].0.translation += (trans1 - trans2).normalize() * to_fix;
                    objects[j].0.translation += (trans2 - trans1).normalize() * to_fix;

                    
                }
            }
        }
    }
    

    pub fn system_set() -> SystemSet {
        SystemSet::new()
            .with_system(Self::apply_gravity.before(Self::apply_physics))
            .with_system(Self::apply_physics)
            .with_system(Self::fix_colisions.before(Self::apply_physics))
    }
}


