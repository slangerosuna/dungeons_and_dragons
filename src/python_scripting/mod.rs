use rustpython_vm::*;
use scripting_module;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start_scripting);
    }
}

pub fn start_scripting() {
    rustpython::run(|vm| {
        vm.add_native_module(
            "ddnd",
            Box::new(scripting_module::make_module)
        );
    });
}


