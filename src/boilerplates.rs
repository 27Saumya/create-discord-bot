pub const PYTHON_MAINFILE_CONTENT: &str =
"import discord
from discord.ext import commands

bot = commands.Bot(command_prefix='!', intents=discord.Intents.default())

@bot.event
async def on_ready():
    print(f'{bot.user.name} has logged in!')


@bot.command()
async def ping(ctx: commands.Context):
    await ctx.send('Pong!')


bot.run('BOT_TOKEN')";

pub const PYTHON_MAINFILE_COG_CONTENT: &str =
"import discord
from discord.ext import commands
from cogs.example import Example


class Bot(commands.Bot):
    def __init__(self):
        super().__init__(command_prefix='!', intents=discord.Intents.default())

        self.add_cog(Example(self))

    async def on_ready(self):
        print(f'{self.user.name} has logged in!')


bot = Bot()

bot.run('TOKEN')";

pub const PYTHON_COG_EXAMPLE_CONTENT: &str =
"from discord.ext import commands

class Example(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.Cog.listener()
    async def on_ready(self):
        print('Example cog has loaded!')

    @commands.command()
    async def ping(self, ctx: commands.Context):
        await ctx.send('Pong!')

def setup(bot):
    bot.add_cog(Example(bot))";

pub const PYTHON_DISNAKE_MAINFILE_CONTENT: &str =
"import disnake
from disnake.ext import commands

intents = disnake.Intents.default()

class Bot(commands.Bot):
    def __init__(self):
        super().__init__(
            command_prefix='!',
            intents=intents
        )

    async def on_ready(self):
        print(f'{self.user.name} has logged in!')

if __name__ == '__main__':
    bot = Bot()
    bot.run('YOUR_BOT_TOKEN')
";

pub const PYTHON_DISNAKE_MAINFILE_COG_CONTENT: &str =
"import disnake
from disnake.ext import commands
from cogs.example import Example

intents = disnake.Intents.default()

class Bot(commands.Bot):
    def __init__(self):
        super().__init__(
            command_prefix='!',
            intents=intents
        )

        self.add_cog(Example(self))

    async def on_ready(self):
        print(f'{self.user.name} has logged in!')

if __name__ == '__main__':
    bot = Bot()
    bot.run('YOUR_BOT_TOKEN')
";

pub const PYTHON_DISNAKE_COG_EXAMPLE_CONTENT: &str =
"from disnake.ext import commands

class Example(commands.Cog):
    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    @commands.Cog.listener()
    async def on_ready(self):
        print('Example cog has loaded!')

    @commands.command()
    async def ping(self, ctx: commands.Context):
        await ctx.send('Pong!')

def setup(bot: commands.Bot):
    bot.add_cog(Example(bot))
";

pub const JS_MAINFILE_CONTENT: &str =
"// Require the necessary discord.js classes
const { Client, Events, GatewayIntentBits } = require('discord.js');
const { token } = require('./config.json');

// Create a new client instance
const client = new Client({ intents: [GatewayIntentBits.Guilds] });

// When the client is ready, run this code (only once)
// We use 'c' for the event parameter to keep it separate from the already defined 'client'
client.once(Events.ClientReady, c => {
    console.log(`Ready! Logged in as ${c.user.tag}`);
});

// Log in to Discord with your client's token
client.login(token);";

pub const JS_MAINFILE_COG_CONTENT: &str =
"// Require the necessary discord.js classes
const { Client, Events, GatewayIntentBits } = require('discord.js');
const { token } = require('./config.json');

// Create a new client instance
const client = new Client({ intents: [GatewayIntentBits.Guilds] });

// Exporting the client which can be used in events/ and commands/
module.exports = client;

// Require all event files
require('./events/example.js')
require('./commands/example.js')

// When the client is ready, run this code (only once)
// We use 'c' for the event parameter to keep it separate from the already defined 'client'
client.once(Events.ClientReady, c => {
    console.log(`Ready! Logged in as ${c.user.tag}`);
});

// Log in to Discord with your client's token
client.login(token);";


pub const CARGO_TOML_CONTENT: &str =
"[package]
name = 'my-bot'
version = '0.1.0'
authors = ['my name <my@email.address>']
edition = '2018'

[dependencies]
serenity = { path = '../../', default-features = false, features = ['client', 'gateway', 'rustls_backend', 'model'] }
tokio = { version = '1.0', features = ['macros', 'rt-multi-thread'] }";

pub const RUST_MAINFILE_CONTENT: &str =
"use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == '!ping' {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, 'Pong!').await {
                println!('Error sending message: {:?}', why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!('{} is connected!', ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var('DISCORD_TOKEN').expect('Expected a token in the environment');
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with 'Bot ', which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect('Err creating client');

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!('Client error: {:?}', why);
    }
}";

pub const RUST_MAINFILE_COG_CONTENT: &str =
"//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = 'https://github.com/serenity-rs/serenity.git'
//! features = ['framework', 'standard_framework']
//! ```
mod commands;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::owner::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!('Connected as {}', ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!('Resumed');
    }
}

#[group]
#[commands(multiply, ping, quit)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect('Failed to load .env file');

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to `debug`.
    tracing_subscriber::fmt::init();

    let token = env::var('DISCORD_TOKEN').expect('Expected a token in the environment');

    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!('Could not access application info: {:?}', why),
    };

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix('~')).group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect('Err creating client');

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect('Could not register ctrl+c handler');
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!('Client error: {:?}', why);
    }
}";

pub const RUST_EXAMPLE_COG_CONTENT: &str =
"use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;

    let product = one * two;

    msg.channel_id.say(&ctx.http, product).await?;

    Ok(())
}";
