use std::env;
use std::sync::Arc;

use ggstdl::Character;
use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

struct DustloopData;

impl TypeMapKey for DustloopData {
    type Value = Arc<RwLock<Vec<Character>>>;
}

#[group]
#[commands(frames, hitboxes)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
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
async fn frames(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let data_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<DustloopData>().expect("Expected Dustloop data in TypeMap.").clone()
    };
    match args.single::<String>() {
        Ok(char_query) => {
            let move_query = args.rest();
            if args.remaining() == 0 || move_query.is_empty() {

                // this lock is dropped when this block exits
                let read = data_lock.read().await;

                let char_search = read.iter().find(|c| c.regex.is_match(char_query.as_str()));
                if let Some(character) = char_search {
                    let move_search = character.moves.iter().find(|m| m.matcher.is_match(move_query));
                    if let Some(move_found) = move_search {
                        msg.reply(ctx, move_found.format(true)).await?;
                    } else {
                        msg.reply(ctx, format!("Could not find move: {}", move_query)).await?;
                    }
                } else {
                    msg.reply(ctx, format!("Could not find character: {}", char_query)).await?;
                }
            } else {
                msg.reply(ctx, "Invalid syntax, try !frames <char> <move_query>").await?;
            }
        },
        Err(_) => {
            msg.reply(ctx, "Invalid syntax, try !frames <char> <move_query>").await?;
        }
    }

    Ok(())
}

#[command]
async fn hitboxes(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.reply(ctx, "In progress!").await?;
    Ok(())
}