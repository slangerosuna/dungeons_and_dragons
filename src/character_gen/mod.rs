use bevy::prelude::*;

pub struct CharacterGenerator;

impl Plugin for CharacterGenerator {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct CharacterInfo {
    //TODO
}

pub struct CharacterBundle (CharacterInfo, SceneBundle);

impl CharacterBundle {
    pub fn generate(generation_prompt: &str) -> Self {
        panic!("Character generation not implemented"); // TODO
    }
}

fn setup() {

}

/* Plan
 * Model Generation:
 *  - Generate a prompt for stable diffusion based on the generation prompt using ChatGPT
 *  - Generate an image based on the prompt using Stable Diffusion
 *  - Generate a 3D model based on the image using DreamGaussian: https://dreamgaussian.github.io/
 *  - Rig the model with RigNet: https://zhan-xu.github.io/rig-net/
 *
 * Model Animation:
 *  - TODO add steps
 *
 * Personality Generation (For NPCs):
 *  - Use ChatGPT to generate NPC traits, bonds, flaws, mannerims (from list of mannerisms we find
 *    animations for), and ideals to create initial personality
 *
 *    Also reference this for character depth & growth: https://arxiv.org/abs/2304.03442
 */
