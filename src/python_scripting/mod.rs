mod scripting_module;

use bevy::prelude::*;

use scripting_module::py_library;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_scripting);
    }
}

pub fn start_scripting() {
    //TODO
}

//TODO add TODOS

/*
 * allow the ai and user to write python scripts to control the game in a homebrew like way
 *    - have system oriented around callbacks for things like spell casting, reactions, etc.
 */
