mod scripting_module;

use bevy::prelude::*;
use rustpython_vm::*;
use scripting_module::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_scripting);
    }
}

pub fn start_scripting() {
    rustpython::run(|vm| {
        vm.add_native_module(
            "ddnd".to_owned(),
            Box::new(scripting_module::make_module)
        );
    });
}

//TODO add TODOS

/**
 * allow the ai and user to write python scripts to control the game in a homebrew like way
 *    - have system oriented around callbacks for things like spell casting, reactions, etc.
 */
