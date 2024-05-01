pub mod sand_sim;
pub mod sand_sim_constants;

use bevy::{prelude::*, window::close_on_esc};
use sand_sim::{spawn_sandbox, SandsimPlugin};
use sand_sim_constants::{SANDBOX_SIZE_HEIGHT, SANDBOX_SIZE_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                title: "Noita-like sand sim".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_plugins(SandsimPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_sandbox(commands, SANDBOX_SIZE_WIDTH, SANDBOX_SIZE_HEIGHT)
}
