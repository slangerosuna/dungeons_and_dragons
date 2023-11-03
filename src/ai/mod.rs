use bevy::prelude::*;
use rs_openai:: {
    OpenAI,
};

pub struct AIPlugin {
    pub api_key: String,
    pub model: String,
}

#[derive(Resource)]
struct AIResource {
    client: OpenAI,
}

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);

        let client = OpenAI::new(&OpenAI {
            api_key: (*self.api_key).to_string(),
            org_id: None, //Some("org-l3pIkFP5UpqbFU68T9Hem30H".to_string()),
        });

        let resource = AIResource {
            client: client,
        };

        app.insert_resource(resource);
    }
}

fn setup(
    mut commands: Commands,
) {
    //TODO all
}

//TODO add script generation

//TODO add npc generation
//TODO add npc communication

//TODO add building reference generation

//TODO add spell icon generation

//TODO add quest generation

//TODO add map generation

//TODO add campaign generation

//TODO add TODOS


//example of how to make an API call
/*
let req = CreateChatRequestBuilder::default()
    .model((*self.model).to_string())
    .messages(vec![ChatCompletionMessageRequestBuilder::default()
        .role(Role::User)
        .name("User".to_string())
        .content("Hello, how are you?")
        .build()?])
    .build()?;

let resp = tokio::runtime::Runtime::new()?.block_on(client.chat().create(&req))?; 
*/
