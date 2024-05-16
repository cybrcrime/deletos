use std::env;
use std::fmt::Debug;

use serenity::all::{CreateAttachment, CreateMessage};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{async_trait, FutureExt};
use tokio::fs::File;
use yt_dlp::yt_dlp;

mod yt_dlp;
struct Handler;

fn wipe_err<T: Debug>(x: T) -> () {
    println!("error:");
    dbg!(x);
    ()
}
impl Handler {
    async fn handle_message(&self, ctx: Context, msg: Message) -> Result<(), ()> {
        let idx = {
            let me = ctx.cache.current_user();
            me.id
        };
        if msg.author.id == idx || msg.embeds.len() == 0 {
            println!("Skipping, no embeds and/or self ping(?)");
            return Ok(());
        }
        if msg.mentions.iter().any(|x| x.id == idx) {
            let reply = msg.reply(&ctx.http, "wait...").await.map_err(wipe_err)?;
            let urls = msg
                .embeds
                .iter()
                .map(|x| dbg!(x))
                .flat_map(|e| e.url.clone())
                .map(|x| dbg!(x))
                .enumerate()
                .map(|(idx, url)| (url, format!("/tmp/{}-{}.mp4", msg.id, idx)))
                .map(|x| dbg!(x))
                .map(|(url, path)| {
                    println!("Downloading {url} to {path}");
                    tokio::spawn(yt_dlp(url, path).then(|x| async {
                        let x = x.map_err(wipe_err)?;
                        let attachment =
                            CreateAttachment::file(&File::open(&x).await.map_err(wipe_err)?, x)
                                .await
                                .map_err(wipe_err)?;
                        Ok::<_, ()>(attachment)
                    }))
                });
            let mut results = Vec::new();

            for res in urls {
                let res = res.await;
                if let Ok(Ok(x)) = res {
                    results.push(x)
                };
            }
            if results.len() == 0 {
                dbg!(reply.delete(&ctx.http).await.map_err(wipe_err).is_ok());
                let x = msg
                    .reply(&ctx.http, "Could not download anything..mb")
                    .await
                    .is_err();
                dbg!(x);
                return Ok(());
            }

            reply.delete(&ctx.http).await.map_err(wipe_err)?;
            let reply = CreateMessage::new()
                .add_files(results)
                .reference_message(&msg);
            msg.channel_id
                .send_message(&ctx.http, reply)
                .await
                .map_err(wipe_err)?;
        };
        Ok(())
    }
}
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if self.handle_message(ctx, msg).await.is_err() {
            println!("failed")
        };
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
