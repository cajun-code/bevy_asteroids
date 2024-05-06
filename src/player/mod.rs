pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        //app.add_system(systems::player_movement_system);
    }
}