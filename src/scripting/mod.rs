use rustpython_vm::*;
use scripting_module;

pub fn main() {
    rustpython::run(|vm| {
        vm.add_native_module(
            "ddnd",
            Box::new(scripting_module::make_module)
        );
    });
}


