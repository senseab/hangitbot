use sea_orm::DbErr;
use teloxide::{
    prelude::*,
    requests::{Request, Requester},
    types::{Message, UpdateKind},
    utils::command::BotCommands,
    RequestError,
};
use wd_log::{log_debug_ln, log_error_ln, log_info_ln, log_panic};

use crate::{
    commands::{CommandHandler, Commands},
    config::Args,
};

pub struct BotServer {
    bot: Bot,
    pub command_handler: CommandHandler,
}

impl BotServer {
    /// Create new bot
    pub async fn new(config: &Args) -> Result<Self, DbErr> {
        Ok(Self {
            bot: Bot::new(config.tgbot_token.to_owned())
                .set_api_url(reqwest::Url::parse(&config.api_url.as_str()).unwrap()),
            command_handler: CommandHandler::new(config).await?,
        })
    }

    fn default_error_hander(&self, error: &RequestError) {
        log_error_ln!("{:?}", error);
    }

    async fn register_commands(&self) {
        if let Err(error) = self
            .bot
            .set_my_commands(Commands::bot_commands())
            .send()
            .await
        {
            self.default_error_hander(&error);
        } else {
            log_info_ln!("commands registered")
        }
    }

    async fn default_update_handler(&self, update_kind: &UpdateKind) {
        log_debug_ln!("non-supported kind {:?}", update_kind);
    }

    fn default_error_handler(&self, error: &RequestError) {
        log_error_ln!("{:?}", error);
    }

    /// Run the bot
    pub async fn run(&self) {
        match self.bot.get_me().send().await {
            Ok(result) => log_info_ln!(
                "connect succeed: id={}, botname=\"{}\"",
                result.id,
                result.username()
            ),
            Err(error) => log_panic!("{}", error),
        }

        self.register_commands().await;

        Commands::repl(
            self.bot.to_owned(),
            |bot: Bot, msg: Message, cmd: Commands| async move {},
        )
        .await;
    }

    /*
    fn answer_generator(&self) -> impl Future<
    Output = fn (Bot, Message, Commands) -> ResponseResult<()>> {
        fn (bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
            async {
                match cmd {
                    Commands::Help => Ok(self.command_handler.help_handler(&bot, &msg).await),
                    Commands::About => Ok(self.command_handler.about_handler(&bot, &msg).await),
                    Commands::Top => Ok(self.command_handler.top_handler(&bot, &msg).await),
                    Commands::HangIt => Ok(self.command_handler.hangit_handler(&bot, &msg).await),
                }
            }
        }
    }
    */
}
