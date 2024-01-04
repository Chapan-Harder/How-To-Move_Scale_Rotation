use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_spawn);
    }
}

fn camera_spawn(mut commands: Commands){
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-4.5, 3.5, -4.5)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}