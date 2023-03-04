use bevy::prelude::*;
use body::DynamicBody;
use collider::grid::Grid;

pub mod camera;
pub mod ball;
pub mod body;
pub mod gravity;
pub mod collider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Grid::new(0.3, 0.2))
        .insert_resource(Time::default())
        .add_system_set(DynamicBody::system_set())
        .add_startup_system(setup)
        .add_startup_system(camera::setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

) {
    // Setup gravity
    let gravity = gravity::Gravity::new(
        Vec3::ZERO,
        1e1);

    commands.spawn(gravity);

    // Spawn 10 OOO balls to a random position in a circle of radius 10
    for _ in 0..20_000 {
        let radius = 0.2;
        let position = Vec3::new(
            (rand::random::<f32>() - 0.5) * 1500.0,
            (rand::random::<f32>() - 0.5) * 1500.0,
            0.0,
        );

        let ball = ball::BallBundle::new(
            &mut meshes,
            &mut materials,
            radius,
            position,
        );

        commands.spawn(ball);
    }
   
}