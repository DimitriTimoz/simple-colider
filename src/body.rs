use std::{ops::Add, collections::HashMap};

use bevy::prelude::*;

use crate::{gravity::Gravity, ball::{Radius, Ball}, collider::grid::{Grid, ColliderDesc}};

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
        mut bodies: Query<(&mut Transform, &mut DynamicBody, &Ball)>,
        time: Res<Time>,
        mut grid: ResMut<Grid>,
    ) {
        grid.clear();
        for (mut transform, mut body, ball) in bodies.iter_mut() {
            let acceleration = body.acceleration.0;
            body.velocity.0 += acceleration * time.delta_seconds();
            transform.translation += body.velocity.0 * time.delta_seconds();
            grid.add(ColliderDesc {
                position: transform.translation.truncate(),
                id: ball.0,
            });
        }
    }

    fn fix_colisions( 
        mut query: Query<(&Ball, &mut Transform, &mut DynamicBody, &Radius, &mut Handle<ColorMaterial>, With<DynamicBody>)>,
        mut grid: ResMut<Grid>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        time: Res<Time>,
    ) {

        let m = query.iter_mut().map(|(ball, transform, body, radius, color, _)| (ball.0, (transform, body, radius.0, color)));
        let mut objects: HashMap<usize, (Mut<Transform>, Mut<DynamicBody>, f32, Mut<Handle<ColorMaterial>>)> = m.collect();
        for cell in grid.get_cells() {
            for cid in cell.get_colliders() {
                let collided = grid.get_collided(cid.position);
                let n_collided = collided.len();
                // Resolve collision
                for c in collided {
                    // Get the Transform and DynamicBody components of the collided entities
                    let mut trans_other = Vec3::ZERO;
                    let mut vel_other = Vec3::ZERO;
                    
                    let mut trans_me = Vec3::ZERO;
                    let mut vel_me = Vec3::ZERO;
    
                    {
                        let cid = cid.id;
                        let me = objects.get(&cid).unwrap();
                        let c_id = c.id;
                        let other = objects.get(&c_id).unwrap();

                        vel_me = -other.1.velocity.0 * 0.5;
                        vel_other = -me.1.velocity.0 * 0.9;

                        // fix position
                        let distance = me.0.translation.distance(other.0.translation);
                        let overlap = 0.5 * (distance - me.2 - other.2);
                        let direction = (other.0.translation - me.0.translation);
                        trans_me = me.0.translation + direction * overlap;
                        trans_other = other.0.translation - direction * overlap;
                    }   
                    // Get mut 
                    {
                        let other = objects.get_mut(&c.id).unwrap();
                        other.0.translation = trans_other;
                        other.1.velocity.0 = vel_other;
                    }
                    {
                        let cid = cid.clone();
                        let me = objects.get_mut(&cid.id).unwrap();
                        me.0.translation = trans_me;
                        me.1.velocity.0 = vel_me;
                    }
                }
                {
        
                    let cid = cid.clone();
                    let me = objects.get_mut(&cid.id).unwrap();
                    /*let mut color_mat = materials.get_mut(&me.3).unwrap();

                    if n_collided > 5 {
                        color_mat.color = Color::rgb(1.0, 0.0, 0.0);
                    } else if n_collided > 1 {
                        color_mat.color = Color::rgb(1.0, 0.0, 1.0);
                    } else {
                        color_mat.color = Color::rgb(0.0, 0.0, 1.0);
                    };*/
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


