mod commands;
mod config;
mod db_controller;
mod inline_query;
mod messages;
mod utils;

use clap::Parser;
use commands::{about_handler, hangit_handler, help_handler, top_handler, Commands};
use config::Args;

use db_controller::Controller;
use inline_query::{inline_anwser, inline_menu};
use teloxide::{
    prelude::*,
    requests::{Request, Requester},
    types::{Me, Update},
    utils::command::BotCommands,
};
use utils::message_handler;
use wd_log::{
    log_debug_ln, log_error_ln, log_info_ln, log_panic, set_level, set_prefix, DEBUG, INFO, log_warn_ln,
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

    let db_controller = match db_controller::Controller::new(args.database_uri.to_owned()).await {
        Ok(db) => db,
        Err(err) => {
            log_panic!("{:?}", err);
        }
    };

    if let Err(err) = db_controller.migrate().await {
        log_panic!("{:?}", err);
    }

    let bot = Bot::new(args.tgbot_token.to_owned())
        .set_api_url(reqwest::Url::parse(&args.api_url.as_str()).unwrap());

    let me = get_me(&bot).await;
    register_commands(&bot).await;

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(dptree::entry().filter_command::<Commands>().endpoint(
                    |db: Controller, black_list: Vec<i64>, bot: Bot, message: Message, cmd: Commands| async move {
                        if black_list.contains(&message.chat.id.0) {
                            log_warn_ln!("banned group dectected: {:?}", message.chat.id);
                            return Ok(());
                        }
                        let r = match cmd {
                            Commands::Help => help_handler(&bot, &message).await,
                            Commands::About => about_handler(&bot, &message).await,
                            Commands::Top => top_handler(&db, &bot, &message).await,
                            Commands::HangIt => hangit_handler(&db, &bot, &message).await,
                        };

                        match r {
                            Ok(_) => Ok(()),
                            Err(err) => {
                                log_error_ln!("{:?}", err);
                                Err(err)
                            }
                        }
                    },
                ))
                .branch(
                    dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
                        .endpoint(|db: Controller, message: Message, me: Me, black_list: Vec<i64>| async move {
                            if black_list.contains(&message.chat.id.0) {
                                log_warn_ln!("banned group dectected: {:?}", message.chat.id);
                                return Ok(());
                            }
                            let r = message_handler(&db, message, &me).await;
                            match r {
                                Ok(_) => Ok(()),
                                Err(err) => {
                                    log_error_ln!("{:?}", err);
                                    Err(err)
                                }
                            }
                        }),
                ),
        )
        .branch(
            Update::filter_inline_query().endpoint(
                |db: Controller, bot: Bot, q: InlineQuery| async move {
                    inline_menu(&db, &bot, q).await
                },
            ),
        )
        .branch(Update::filter_chosen_inline_result().endpoint(
            |db: Controller, a: ChosenInlineResult| async move { inline_anwser(&db, a).await },
        ));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db_controller, me, args.group_banned])
        .default_handler(|upd| async move { log_debug_ln!("unhandled update: {:?}", upd) })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn register_commands(bot: &Bot) {
    if let Err(error) = bot.set_my_commands(Commands::bot_commands()).send().await {
        log_error_ln!("{:?}", error);
    } else {
        log_info_ln!("commands registered")
    }
}

async fn get_me(bot: &Bot) -> Me {
    match bot.get_me().send().await {
        Ok(result) => {
            log_info_ln!(
                "connect succeed: id={}, botname=\"{}\"",
                result.id,
                result.username()
            );
            result
        }
        Err(error) => log_panic!("{}", error),
    }
}
