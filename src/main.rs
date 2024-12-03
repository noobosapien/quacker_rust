mod setup;

use bevy::prelude::*;

use setup::setup::camera_setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Quacker".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (camera_setup).chain())
        .run();
}
