mod player;
mod database;
mod telegram;


use anyhow::Result;

use crate::player::Player;
use crate::telegram::{Bot, Message};


fn main() -> Result<()> {
    let bot: Bot = bot!(std::env::var("BOT_TOKEN")?);
    bot.start(process_message)?;
    Ok(())
}


fn process_message(message: Message) -> Option<Result<(String, String)>> {
    Some(Ok((message.text, message.from.id.to_string())))
}
