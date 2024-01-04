use bevy::prelude::*;

use std::f32::consts::TAU;

#[derive(Component)]
pub struct Movement{
    pub spawn: Vec3,
    pub max_distance: f32,
    pub speed: f32
}

#[derive(Component)]
pub struct Spin{
    pub speed: f32
}

#[derive(Component)]
pub struct Scaling{
    pub scale_direction: Vec3,
    pub scale_speed: f32,
    pub max_element_size: f32,
    pub min_element_size: f32
}

impl Movement{
    pub fn new(spawn: Vec3) -> Self{
        Movement{
            spawn,
            max_distance: 1.5,
            speed: 2.0
        }
    }
}

impl Scaling{
    pub fn new() -> Self{
        Scaling{
            scale_direction: Vec3::X,
            scale_speed: 3.0,
            max_element_size: 3.0,
            min_element_size: 1.0
        }
    }
}

pub struct MovePlugin;

impl Plugin for MovePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (movement, rotation, change_scale_direction, scale));
    }
}

// Move
fn movement(mut queries: Query<(&mut Transform, &mut Movement)>, time: Res<Time>){
    for (mut transform, mut query) in &mut queries{
        if (query.spawn - transform.translation).length() > query.max_distance{
            query.speed *= -1.0;
        }
        let direction = transform.local_z();
        transform.translation += direction * query.speed * time.delta_seconds();
    }
}

// Rotation
fn rotation(mut queries: Query<(&mut Transform, &Spin)>, time: Res<Time>){
    for (mut transform, query) in &mut queries{
        transform.rotate_y(query.speed * TAU * time.delta_seconds());
    }
}

// Scale
fn change_scale_direction(mut query: Query<(&mut Transform, &mut Scaling)>) {
    for (mut transform, mut cube) in &mut query {
        if transform.scale.max_element() > cube.max_element_size {
            cube.scale_direction *= -1.0;
            transform.scale = transform.scale.floor();
        }
        if transform.scale.min_element() < cube.min_element_size {
            cube.scale_direction *= -1.0;
            transform.scale = transform.scale.ceil();
            cube.scale_direction = cube.scale_direction.zxy();
        }
    }
}

fn scale(mut queries: Query<(&mut Transform, &Scaling)>, time: Res<Time>){
    for (mut transform, query) in &mut queries{
        transform.scale += query.scale_direction * query.scale_speed * time.delta_seconds(); 
    }
}