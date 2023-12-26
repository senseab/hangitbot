mod commands;
mod config;
mod db_controller;
mod messages;

use clap::Parser;
use commands::{CommandHandler, Commands};
use config::Args;

use teloxide::{
    prelude::*,
    requests::{Request, Requester},
    utils::command::BotCommands,
};
use wd_log::{
    log_debug_ln, log_error_ln, log_info_ln, log_panic, set_level, set_prefix, DEBUG, INFO,
};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    set_prefix("hangitboot");

    if args.debug {
        set_level(DEBUG);
        log_debug_ln!("{:?}", args);
    } else {
        set_level(INFO);
    }
    let command_handler = match CommandHandler::new(&args).await {
        Err(err) => log_panic!("{}", err),
        Ok(c) => c,
    };

    command_handler.init().await.unwrap();

    let bot = Bot::new(args.tgbot_token.to_owned())
        .set_api_url(reqwest::Url::parse(&args.api_url.as_str()).unwrap());

    get_me(&bot).await;
    register_commands(&bot).await;

    Commands::repl(bot, move |bot: Bot, message: Message, cmd: Commands| {
        let command_handler = command_handler.clone();
        async move {
            let _ = match cmd {
                Commands::Help => command_handler.clone().help_handler(&bot, &message).await,
                Commands::About => command_handler.clone().about_handler(&bot, &message).await,
                Commands::Top => command_handler.clone().top_handler(&bot, &message).await,
                Commands::HangIt => command_handler.clone().hangit_handler(&bot, &message).await,
            };
            Ok(())
        }
    })
    .await;
}

async fn register_commands(bot: &Bot) {
    if let Err(error) = bot.set_my_commands(Commands::bot_commands()).send().await {
        log_error_ln!("{:?}", error);
    } else {
        log_info_ln!("commands registered")
    }
}

async fn get_me(bot: &Bot) {
    match bot.get_me().send().await {
        Ok(result) => log_info_ln!(
            "connect succeed: id={}, botname=\"{}\"",
            result.id,
            result.username()
        ),
        Err(error) => log_panic!("{}", error),
    }
}