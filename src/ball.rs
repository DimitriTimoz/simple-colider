use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::body::{DynamicBody, Velocity};

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Bundle)]
pub struct BallBundle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub ball: Ball,
    pub radius: Radius,
    pub body: DynamicBody,
}

impl BallBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        position: Vec3,
    ) -> Self {
        // Get a random color
        let mut rng = rand::thread_rng();
        let color = Color::rgb(rng.gen(), rng.gen(), rng.gen());
        
        Self {
            mesh: MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(color)),               
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            ball: Ball,
            radius: Radius(radius),
            body: DynamicBody {
                velocity: Velocity(Vec3::new(
                    (rng.gen::<f32>() - 0.5) * 2.0,
                    (rng.gen::<f32>() - 0.5) * 2.0,
                    0.0,
                )),
                ..Default::default()
            }
        }
    }
}
