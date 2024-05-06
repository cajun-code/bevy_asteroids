use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: asset_server.load("PNG/playerShip1_blue.png"),
            ..default()
        },
        Player{},
    ));
}