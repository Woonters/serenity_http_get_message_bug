use serenity::{
    self,
    all::{ChannelId, Context, EventHandler, GatewayIntents, Message, MessageId},
    async_trait, Client,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, _: Message) {
        // PUT CHANNEL AND MESSAGE IDs HERE

        let reactless_channel_id = ChannelId::new();
        let reactless_message_id = MessageId::new(); // THIS MESSAGE SHOULD HAVE NO REACT IMAGES

        let reacted_channel_id = ChannelId::new();
        let reacted_message_id = MessageId::new(); // THIS MESSAGE SHOULD HAVE SOME REACT EMOTES (CUSTOM OR DEFAULT)
        let reactless_got_message = ctx
            .http
            .get_message(reactless_channel_id, reactless_message_id)
            .await;
        let reacted_got_message = ctx
            .http
            .get_message(reacted_channel_id, reacted_message_id)
            .await;
        match reactless_got_message {
            Ok(msg) => println!("Got the message with the text: {}", msg.content), // This is the branch that will be hit
            Err(e) => println!("There was an error getting the message: {}", e),
        };
        match reacted_got_message {
            Ok(msg) => println!("Got the message with the text: {}", msg.content),
            Err(e) => println!("There was an error getting the message: {}", e), // this is the branch that will be hit
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing token");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Couldn't create client");
    let _ = client.start().await;

    println!("Hello, world!");
}
