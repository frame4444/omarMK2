mod actions;
mod handlers;
mod models;

use handlers::handle_message;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // carga el .env al entorno del proceso
    pretty_env_logger::init();
    let asamblea_chat: i64 = std::env::var("ASAMBLEA_CHAT_ID")
        .expect("id not in chat")
        .parse()
        .expect("id not a number");
    let bot_name = std::env::var("BOT_NAME").unwrap();
    let bot = Bot::from_env();

    log::info!("{} has awaken!", bot_name);
    bot.send_message(ChatId(asamblea_chat), format!("{} back online", bot_name))
        .await
        .expect("");
    bot.send_message(ChatId(asamblea_chat), "blackass::video.service back online")
        .await
        .expect("");

    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        handle_message(bot, msg).await
    })
    .await;
}
