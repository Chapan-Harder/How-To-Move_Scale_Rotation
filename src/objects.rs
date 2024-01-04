use bevy::prelude::*;

use crate::movescalerotation::{Movement, Spin, Scaling};
pub struct CubesPlugin;

impl Plugin for CubesPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (cubes_spawn, test_ground));
    }
}

fn test_ground(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: ResMut<AssetServer>){
    // Ground
    let ground: Handle<Mesh> = meshes.add(shape::Plane{size: 30.0, subdivisions: 0}.into());

    // Spawn The Ground
    commands.spawn(PbrBundle{
        mesh: ground.clone(),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("textures/Blue ground test.png")),
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn cubes_spawn(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: ResMut<AssetServer>){
    // Cubes
    let mesh1: Handle<Mesh> = meshes.add(shape::Cube{size: 1.0}.into());
    let mesh2: Handle<Mesh> = meshes.add(shape::Cube{size: 1.0}.into());
    let mesh3: Handle<Mesh> = meshes.add(shape::Cube{size: 1.0}.into());
    
    // Locations
    let transform_mesh1: Vec3 = Vec3{x: -2.0, y: 0.5, z: 2.0};
    let transform_mesh2: Vec3 = Vec3{x: 0.0, y: 0.5, z: 0.0};
    let transform_mesh3: Vec3 = Vec3{x:2.0, y: 1.5, z: -2.0};

    // Spawn The Cubes
    commands.spawn((PbrBundle{
        mesh: mesh1.clone(),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("textures/White ground test.png")),
            ..Default::default()
        }),
        transform: Transform::from_translation(transform_mesh1),
        ..Default::default()
    },
    Movement::new(transform_mesh1)
));

    commands.spawn((PbrBundle{
        mesh: mesh2.clone(),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("textures/White ground test.png")),
            ..Default::default()
        }),
        transform: Transform::from_translation(transform_mesh2),
        ..Default::default()
    },
    Spin{speed: 0.7}
));

    commands.spawn((PbrBundle{
        mesh: mesh3.clone(),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("textures/White ground test.png")),
            ..Default::default()
        }),
        transform: Transform::from_translation(transform_mesh3),
        ..Default::default()
    },
    Scaling::new()
));
}