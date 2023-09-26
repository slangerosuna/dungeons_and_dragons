use bevy::prelude::*;
use rustpython_vm:: {
    pymodule,
    function::{
        ArgCallable,
    },
    builtins::{
        PyStrRef,
    },
};
use crate::dnd_structs::ResourceType;

#[derive(Component)]
pub struct Spell<'a>{
    required_resources: Vec<ResourceType>,

    name: String,
    icon_path: String, //path to the image displayed to represent the spell
    description: String,

    cast: &'a ArgCallable, //MAKE SURE TO HANDLE LIFETIMES PROPERLY 
    break_concentration: &'a ArgCallable, //MAKE SURE TO HANDLE LIFETIMES PROPERLY

    per_long_rest: bool,
    per_short_rest: bool,

    used_this_rest: bool,
}

impl Spell<'_> {
    fn new(
        required_resources: Vec<ResourceType>,
        
        name: String,
        icon_path: String,
        description: String,
        
        cast: &ArgCallable,
        break_concentration: &ArgCallable,

        per_long_rest: bool,
        per_short_rest: bool,
    ) -> Self {
        Self {
            required_resources: required_resources,
            
            name: name,
            icon_path: icon_path,
            description: description,
            
            cast: cast,
            break_concentration: break_concentration,

            per_long_rest: per_long_rest,
            per_short_rest: per_short_rest,

            used_this_rest: false,
        }
    }

    //TODO cast function
    
    //TODO break_concentration function
}

//TODO make resource for casting spells

#[pymodule]
pub mod scripting_module {
    //TODO all
}
