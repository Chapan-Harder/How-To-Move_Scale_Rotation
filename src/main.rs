mod camera;
mod objects;
mod lights;
mod movescalerotation;

use bevy::prelude::*;

use camera::CameraPlugin;
use objects::CubesPlugin;
use lights::LightsPlugin;
use movescalerotation::MovePlugin;

fn main() {
    App::new()
    .add_plugins(MovePlugin)
    .add_plugins(LightsPlugin)
    .add_plugins(CubesPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(DefaultPlugins)
    .run()
}
