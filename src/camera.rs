use bevy::prelude::*;


pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection : OrthographicProjection {
            scale: 1.0/40.0,
            ..Default::default()
        },
        ..Default::default()
    });
}