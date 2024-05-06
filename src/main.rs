pub mod game;
pub mod player;

use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (player::systems::spawn_player, spawn_camera))
        .run();

}


pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().expect("no primary window");
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}