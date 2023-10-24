use bevy::prelude::*;

mod dnd_structs;
mod networking;
mod ai;
mod python_scripting;
mod building_gen;
mod terrain_gen;

use dnd_structs::*;
use bevy_steamworks::*;
use networking::*;
use ai::*;
use python_scripting::*;
use building_gen::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BuildingGenerator)
        .add_plugin(NetworkingPlugin{
            max_players: 4, max_synced_objects: 1000, 
            app_id: 480, packet_per_frame_limit: 255,
        })
        .add_plugin(ScriptingPlugin)
        .add_plugin(AIPlugin {
            api_key: String::from(
                "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"
            ),
            model: String::from("gpt-3.5-turbo"),
        })
        .run();
}
