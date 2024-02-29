use teloxide::{
    prelude::Bot,
    requests::{Request, Requester},
    types::{
        ChatId, ChosenInlineResult, InlineQuery, InlineQueryResult, InlineQueryResultArticle,
        InputMessageContent, InputMessageContentText,
    },
    RequestError,
};
use wd_log::{log_debug_ln, log_error_ln};

use crate::{
    db_controller::Controller,
    messages::BOT_TEXT_INLINE_HANG,
    utils::{hangit_text, IS_SELF, NEED_ESCAPE},
};

pub async fn inline_menu(db: &Controller, bot: &Bot, q: InlineQuery) -> Result<(), RequestError> {
    let name = q.query;

    let mut results = match db.find_by_name(&name).await {
        Some(list) => list
            .iter()
            .map(|n| {
                InlineQueryResult::Article(InlineQueryResultArticle::new(
                    format!("{},{}", n.id, n.name),
                    format!("{} {}", BOT_TEXT_INLINE_HANG, n.name),
                    InputMessageContent::Text(InputMessageContentText::new(hangit_text(
                        n.name.clone(),
                        q.from.first_name == n.name,
                        !NEED_ESCAPE,
                    ))),
                ))
            })
            .collect::<Vec<_>>(),

        None => vec![],
    };

    log_debug_ln!("{:?}", results);

    if results.is_empty() {
        results.push(InlineQueryResult::Article(InlineQueryResultArticle::new(
            format!("{},{}", 0, name),
            format!("{} {}", BOT_TEXT_INLINE_HANG, name.clone()),
            InputMessageContent::Text(InputMessageContentText::new(hangit_text(
                name.clone(),
                !IS_SELF,
                !NEED_ESCAPE,
            ))),
        )));
    }

    if name.starts_with("@") {
        results = vec![]
    }

    bot.answer_inline_query(&q.id, results).send().await?;
    Ok(())
}

pub async fn inline_anwser(db: &Controller, a: ChosenInlineResult) -> Result<(), RequestError> {
    log_debug_ln!("{:#?}", a);

    let mut c: Vec<&str> = a.result_id.split(",").collect();
    c.remove(0);
    let result_id = c.concat();

    if result_id == "@" {
        return Ok(());
    }

    if let Err(err) = db.hangit(&result_id, ChatId(0)).await {
        log_error_ln!("{:?}", err);
    }

    Ok(())
}
