use bevy::prelude::*;

pub struct LightsPlugin;

impl Plugin for LightsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, lamps);
    }
}

fn lamps(mut commands: Commands){
    let position_of_lamp1: Transform = Transform::from_xyz(-2.0, 7.0, 2.0);
    let position_of_lamp2: Transform = Transform::from_xyz(0.0, 7.0, 0.0);
    let position_of_lamp3: Transform = Transform::from_xyz(2.0, 7.0, -2.0);

    commands.spawn(SpotLightBundle{
        spot_light: SpotLight{
            intensity: 1000.0,
            range: 100.0,
            radius: 1.5,
            outer_angle: 0.20,
            inner_angle: 0.15,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: position_of_lamp1.looking_at(Vec3{x: -2.0, y: 0.0, z: 2.0}, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SpotLightBundle{
        spot_light: SpotLight{
            intensity: 1000.0,
            range: 100.0,
            radius: 1.5,
            outer_angle: 0.20,
            inner_angle: 0.15,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: position_of_lamp2.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SpotLightBundle{
        spot_light: SpotLight{
            intensity: 1000.0,
            range: 100.0,
            radius: 1.5,
            outer_angle: 0.20,
            inner_angle: 0.15,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: position_of_lamp3.looking_at(Vec3{x: 2.0, y: 0.0, z: -2.0}, Vec3::Y),
        ..Default::default()
    });
}