use rand::Rng;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::env;
use std::fs::File;
#[group]
#[commands(quote)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(">")) // set the bot's prefix to ">"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = &env::var("DISCORD_TOKEN").unwrap(); // get the token from the environment
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    let file = File::open("src/philososhark.json").expect("file failed to open!");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON!");
    let index = rand::thread_rng().gen_range(0..json.as_array().unwrap().len() + 1);
    let quote = format!(
        "
```
         ({quote} - {author})
         .     /
  \\_____)\\_____
   /--v____ __`<
           )/
           '
```
        ",
        quote = &json[index]["text"],
        author = &json[index]["author"].to_string().replace("\"", "")
    );
    msg.reply(ctx, quote).await?;
    Ok(())
}
