use std::{collections::HashMap, error::Error};

use rand::{rngs::OsRng, Rng};
use sea_orm::DbErr;
use strfmt::Format;
use teloxide::{
    payloads::{SendMessage, SendMessageSetters},
    prelude::Bot,
    requests::{JsonRequest, Requester},
    types::{Message, ParseMode},
    utils::{command::BotCommands, markdown::escape},
};
use wd_log::log_error_ln;

use crate::{
    config::Args,
    db_controller::Controller,
    messages::{
        BOT_ABOUT, BOT_TEXT_HANGED, BOT_TEXT_IS_CHANNEL, BOT_TEXT_NO_TARGET, BOT_TEXT_TOP_GLOBAL,
        BOT_TEXT_TOP_GROUP, BOT_TEXT_TOP_NONE, BOT_TEXT_TOP_TEMPLATE, BOT_TEXT_TOP_TITLE,
    },
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

#[derive(Debug, Clone)]
pub struct CommandHandler {
    pub controller: Controller,
}

impl CommandHandler {
    pub async fn new(config: &Args) -> Result<Self, DbErr> {
        Ok(Self {
            controller: Controller::new(config.database_uri.to_owned()).await?,
        })
    }

    pub async fn init(&self) -> Result<(), DbErr> {
        self.controller.migrate().await
    }

    async fn send_text_reply(
        &self,
        bot: &Bot,
        message: &Message,
        text: String,
    ) -> JsonRequest<SendMessage> {
        bot.send_message(message.chat.id, text)
            .reply_to_message_id(message.id)
            .parse_mode(ParseMode::MarkdownV2)
    }

    pub async fn help_handler(
        &self,
        bot: &Bot,
        message: &Message,
    ) -> Result<JsonRequest<SendMessage>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .send_text_reply(bot, message, Commands::descriptions().to_string())
            .await)
    }

    pub async fn about_handler(
        &self,
        bot: &Bot,
        message: &Message,
    ) -> Result<JsonRequest<SendMessage>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .send_text_reply(bot, message, BOT_ABOUT.to_string())
            .await)
    }

    pub async fn hangit_handler(
        &self,
        bot: &Bot,
        message: &Message,
    ) -> Result<JsonRequest<SendMessage>, Box<dyn Error + Send + Sync>> {
        let reply = match message.reply_to_message() {
            Some(reply) => reply,
            None => {
                return Ok(self
                    .send_text_reply(bot, message, BOT_TEXT_NO_TARGET.to_string())
                    .await)
            }
        };

        match reply.from() {
            Some(user) => {
                let mut vars = HashMap::new();
                let index = OsRng.gen::<usize>() % BOT_TEXT_HANGED.len();
                let text = BOT_TEXT_HANGED[index];

                vars.insert("name".to_string(), user.first_name.as_str());

                let _ = self
                    .controller
                    .hangit(&user.full_name(), message.chat.id)
                    .await;
                Ok(self
                    .send_text_reply(bot, reply, escape(&text.format(&vars).unwrap()))
                    .await)
            }
            None => Ok(self
                .send_text_reply(bot, message, BOT_TEXT_IS_CHANNEL.to_string())
                .await),
        }
    }

    pub async fn top_handler(
        &self,
        bot: &Bot,
        message: &Message,
    ) -> Result<JsonRequest<SendMessage>, Box<dyn Error + Send + Sync>> {
        let chat_id = message.chat.id;
        let scope = match chat_id.is_group() {
            true => BOT_TEXT_TOP_GROUP,
            false => BOT_TEXT_TOP_GLOBAL,
        };

        let mut index = 1;
        let mut text = format!("{}-{}\n\n", BOT_TEXT_TOP_TITLE, scope);
        let results = match self.controller.top(chat_id).await {
            Ok(r) => match r {
                Some(result) => result,
                None => {
                    return Ok(self
                        .send_text_reply(bot, message, BOT_TEXT_TOP_NONE.to_string())
                        .await)
                }
            },
            Err(error) => {
                log_error_ln!("{}", error);
                return Err(Box::new(error));
            }
        };

        for result in results {
            let mut vars = HashMap::new();

            vars.insert("name".to_string(), result.name);
            vars.insert("count".to_string(), result.counts.to_string());

            let record = BOT_TEXT_TOP_TEMPLATE.format(&vars).unwrap();

            text = format!("{}{} {}\n", text, index, record);
            index += 1;
        }

        Ok(self.send_text_reply(bot, message, text).await)
    }
}
