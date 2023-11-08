use bevy::prelude::*;
use crate::python_scripting::*;
use pyo3::{*, types::*,};
pub enum Resource {
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
pub struct Move { //Action, BonusAction, Reaction, etc.
    required_resources: Vec<Resource>,

    name: String,
    icon_path: String, //path to the image displayed to represent the spell
    description: String,

    cast: PyFn,
    break_concentration: PyFn,

    per_long_rest: bool,
    per_short_rest: bool,

    used_this_rest: bool,
}

impl Move {
    fn new(
        required_resources: Vec<Resource>,
        
        name: String,
        icon_path: String,
        description: String,
        
        cast: PyFn, 
        break_concentration: PyFn,

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
    fn cast(&mut self, args: &impl IntoPy<Py<PyTuple>>) /*-> TODO*/ {
        //TODO check if the spell can be cast   
        
        //TODO cast the spell on python thread
    }

    //TODO break_concentration function
    fn break_concentration(&mut self, args: &impl IntoPy<Py<PyTuple>>) /*-> TODO*/ {
        //TODO check if concentration should be broken
        
        //TODO break concentration on python thread
    }
}
//TODO add TODOS

//TODO make resource for casting spells
/*
 * this module is for all the structs that are parts of dnd and don't fall particularly well into any other module, like a util module
 */
