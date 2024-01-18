use std::collections::HashMap;

use strfmt::Format;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Bot,
    requests::Requester,
    types::{Message, ParseMode},
    utils::{command::BotCommands, markdown::escape},
    RequestError,
};

use crate::{
    db_controller::Controller,
    messages::{
        BOT_ABOUT, BOT_TEXT_HANG_ANONYMOUS, BOT_TEXT_HANG_BOT, BOT_TEXT_HANG_CHANNEL,
        BOT_TEXT_IS_CHANNEL, BOT_TEXT_NO_TARGET, BOT_TEXT_TOP_GLOBAL, BOT_TEXT_TOP_GROUP,
        BOT_TEXT_TOP_NONE, BOT_TEXT_TOP_TEMPLATE, BOT_TEXT_TOP_TITLE,
    },
    utils::hangit_text,
};

#[derive(BotCommands, PartialEq, Debug, Clone)]
#[command(rename_rule = "lowercase", description = "帮助：")]
pub enum Commands {
    #[command(description = "显示帮助信息")]
    Help,

    #[command(description = "关于本 Bot")]
    About,

    #[command(description = "排行榜")]
    Top,

    #[command(description = "吊丫起来！")]
    HangIt,
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Help
    }
}

async fn send_text_reply(bot: &Bot, message: &Message, text: String) -> Result<(), RequestError> {
    bot.send_message(message.chat.id, text)
        .reply_to_message_id(message.id)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub async fn help_handler(bot: &Bot, message: &Message) -> Result<(), RequestError> {
    send_text_reply(bot, message, Commands::descriptions().to_string()).await
}

pub async fn about_handler(bot: &Bot, message: &Message) -> Result<(), RequestError> {
    send_text_reply(bot, message, BOT_ABOUT.to_string()).await
}

pub async fn hangit_handler(
    db: &Controller,
    bot: &Bot,
    message: &Message,
) -> Result<(), RequestError> {
    let reply = match message.reply_to_message() {
        Some(reply) => reply,
        None => {
            return send_text_reply(bot, message, BOT_TEXT_NO_TARGET.to_string()).await;
        }
    };

    match reply.from() {
        Some(user) => {
            if user.is_bot {
                return send_text_reply(bot, message, BOT_TEXT_HANG_BOT.to_string()).await;
            }

            if user.is_anonymous() {
                return send_text_reply(bot, message, BOT_TEXT_HANG_ANONYMOUS.to_string()).await;
            }

            if user.is_channel() {
                return send_text_reply(bot, message, BOT_TEXT_HANG_CHANNEL.to_string()).await;
            }

            let is_self = match message.from() {
                Some(f) => f.first_name == user.first_name,
                None => false,
            };

            let _ = db.hangit(&user.full_name(), message.chat.id).await;
            send_text_reply(
                bot,
                reply,
                hangit_text(user.first_name.to_string(), is_self, true),
            )
            .await
        }
        None => send_text_reply(bot, message, BOT_TEXT_IS_CHANNEL.to_string()).await,
    }
}

pub async fn top_handler(
    db: &Controller,
    bot: &Bot,
    message: &Message,
) -> Result<(), RequestError> {
    let chat = &message.chat;
    let scope = match chat.is_group() || chat.is_supergroup() {
        true => BOT_TEXT_TOP_GROUP,
        false => BOT_TEXT_TOP_GLOBAL,
    };

    let mut index = 1;
    let mut text = format!("{}\\-{}\n\n", BOT_TEXT_TOP_TITLE, scope);
    let results = match db.top(chat).await {
        Some(result) => result,
        None => {
            return send_text_reply(bot, message, BOT_TEXT_TOP_NONE.to_string()).await;
        }
    };

    for result in results {
        let mut vars: HashMap<String, String> = HashMap::new();

        vars.insert("name".to_string(), escape(result.name.as_str()));
        vars.insert("count".to_string(), result.counts.to_string());

        let record = BOT_TEXT_TOP_TEMPLATE.format(&vars).unwrap();

        text = format!("{}{}\\. {}\n", text, index, record);
        index += 1;
    }

    send_text_reply(bot, message, text).await
}
