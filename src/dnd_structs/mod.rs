use rustpython_vm::function::ArgCallable;
use bevy::prelude::*;

pub enum ResourceType {
    Action,
    BonusAction,
    Reaction,
    SpellSlot1,
    SpellSlot2,
    SpellSlot3,
    SpellSlot4,
    SpellSlot5,
    SpellSlot6,
    SpellSlot7,
    SpellSlot8,
    SpellSlot9,
}

#[derive(Component)]
pub struct Spell {
    required_resources: Vec<ResourceType>,

    name: String,
    icon_path: String, //path to the image displayed to represent the spell
    description: String,

    cast: Box<ArgCallable>,
    break_concentration: Box<ArgCallable>,

    per_long_rest: bool,
    per_short_rest: bool,

    used_this_rest: bool,
}

impl Spell {
    fn new(
        required_resources: Vec<ResourceType>,
        
        name: String,
        icon_path: String,
        description: String,
        
        cast: Box<ArgCallable>, 
        break_concentration: Box<ArgCallable>,

        per_long_rest: bool,
        per_short_rest: bool,
    ) -> Spell {
        Spell {
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
//TODO add TODOS

//TODO make resource for casting spells
/*
 * this module is for all the structs that are parts of dnd and don't fall particularly well into any other module, like a util module
 */
