use bevy::prelude::*;
use rs_openai:: {
    chat::{
        ChatCompletionMessageRequestBuilder,
        CreateChatRequestBuilder,
        Role,
    },
    OpenAI,
};
use tokio::runtime::Runtime;

pub struct AIPlugin {
    pub api_key: String,
    pub model: String,
}

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        /*app
            .add_startup_system(setup.system());*/

        let res = move || -> Result<(), Box<dyn std::error::Error>> { 
            let client = OpenAI::new(&OpenAI {
                api_key: (*self.api_key).to_string(),
                org_id: None,
            });

            let req = CreateChatRequestBuilder::default()
                .model("self.model")
                .messages(vec![ChatCompletionMessageRequestBuilder::default()
                    .role(Role::User)
                    .content("Hello, how are you?")
                    .build()?])
                .build()?;

            let resp = Runtime::new()?.block_on(client.chat().create(&req))?;
            println!("{:#?}", resp);

            Ok(())
        }();

        if let Err(e) = res {
            eprintln!("Error: {}", e);
        }
    }
}

pub struct AIResource {
    chat_request_builder: CreateChatRequestBuilder,
}

fn setup(mut commands: Commands) {

}
