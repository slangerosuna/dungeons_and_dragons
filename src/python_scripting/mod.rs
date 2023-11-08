use bevy::prelude::*;
use std::sync::Mutex;
use std::thread::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_scripting);
    }
}

#[derive(Resource)]
pub struct ScriptingResource;

impl ScriptingResource {
    pub fn register_func(
        &self,
        //TODO
    ) -> PyFn {
        //TODO send function channels to scripting thread
    }
}

pub struct PyFn {
    //TODO use channels in order to have python calls on scripting thread
}

impl PyFn {
    fn call (
        &mut self,
        //TODO
    ) {
        //TODO
    }
}

pub fn start_scripting(
    mut commands: Commands,
) {
    //TODO MAKE RESOURCE
    spawn(
        move || {
            //TODO python managing thread
            panic!("Not Implemented yet"); //Prevents empty infinite loop
            loop {
                //TODO find when to break
            }
        }
    );
}

//TODO add TODOS

/*
 * allow the ai and user to write python scripts to control the game in a homebrew like way
 *    - have system oriented around callbacks for things like spell casting, reactions, etc.
 */
