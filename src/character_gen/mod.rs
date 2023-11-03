use bevy::prelude::*;

pub struct CharacterGenerator;

impl Plugin for CharacterGenerator {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct CharacterInfo {
    
}

pub struct CharacterBundle (CharacterInfo);

impl CharacterBundle {
    pub fn new(generation_prompt: &str) -> Self {
        CharacterBundle(CharacterInfo {
            
        })
    }
}

fn setup() {

}
