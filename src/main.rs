use bevy::prelude::*;

mod dnd_structs;
mod networking;
mod ai;
mod python_scripting;
mod building_gen;
mod terrain_gen;
mod character_gen;

use character_gen::*;
use networking::*;
use ai::*;
use python_scripting::*;
use building_gen::*;

fn main() {
    //Makes it so that if someone calls `panic!("at the disco")` it will open a Panic! at the Disco song
    //This has no real purpose, but I find it funny and it's my project so you can't stop me
    std::panic::set_hook(Box::new(|panic_info| {
        if Some(&"at the disco") == panic_info.payload().downcast_ref::<&str>() {
           open::that("https://www.youtube.com/watch?v=H5NqIsnyTG8").ok();
        }
    }));
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterGenerator)
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
