mod networking;
mod ai;
mod building_gen;
mod terrain_gen;

use bevy::prelude::*;
use bevy_steamworks::*;
use networking::*;
use ai::*;

fn main() {
    App::new()
        .add_plugin(AIPlugin {
            api_key: String::from(""),
            model: String::from("gpt-3.5-turbo"),
        })
        .run();
}
