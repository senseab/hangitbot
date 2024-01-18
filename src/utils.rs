use std::collections::HashMap;

use rand::{rngs::OsRng, Rng};
use regex::Regex;
use strfmt::Format;
use teloxide::{
    types::{Me, Message},
    utils::markdown::escape,
    RequestError,
};
use wd_log::{log_debug_ln, log_error_ln};

use crate::{
    db_controller::Controller,
    messages::{BOT_TEXT_HANGED, BOT_TEXT_HANGED_SELF},
};

pub const IS_SELF: bool = true;
pub const NEED_ESCAPE: bool = true;

pub fn hangit_text(name: String, is_self: bool, need_escape: bool) -> String {
    let mut vars = HashMap::new();
    let index = if is_self {
        OsRng.gen::<usize>() % BOT_TEXT_HANGED_SELF.len()
    } else {
        OsRng.gen::<usize>() % BOT_TEXT_HANGED.len()
    };

    let text = if is_self {
        BOT_TEXT_HANGED_SELF[index]
    } else {
        BOT_TEXT_HANGED[index]
    };

    let name = if need_escape {
        escape(name.as_str())
    } else {
        name
    };
    vars.insert("name".to_string(), name.as_str());

    text.format(&vars).unwrap()
}

pub async fn message_handler(db: &Controller, msg: Message, me: &Me) -> Result<(), RequestError> {
    let text = match msg.text() {
        Some(t) => t.to_owned(),
        None => {
            log_debug_ln!("{:?}", msg);
            return Ok(());
        }
    };

    let formats = vec![BOT_TEXT_HANGED.to_vec(), BOT_TEXT_HANGED_SELF.to_vec()]
        .concat()
        .iter()
        .map(|i| Regex::new(&format!("^{}$", i.replace("{name}", "(.+)"))).unwrap())
        .collect::<Vec<_>>();

    if let Some(via_bot) = msg.via_bot {
        if via_bot.is_bot && via_bot.id == me.id {
            for f in formats {
                log_debug_ln!("Regexp {:?}", f);
                if !f.is_match(text.as_str()) {
                    continue;
                }

                if let Some(cap) = f.captures(text.as_str()) {
                    if let Some(name) = cap.get(1) {
                        log_debug_ln!("got username: {:?}", name.as_str());
                        if let Err(error) = db
                            .update_group(name.as_str().to_string(), msg.chat.id)
                            .await
                        {
                            log_error_ln!("{:?}", error);
                        }
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
