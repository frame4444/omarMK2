mod actions;
mod handlers;
mod models;

use handlers::handle_message;
use teloxide::prelude::*;
use teloxide::update_listeners::Polling;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // carga el .env al entorno del proceso
    pretty_env_logger::init();
    let _asamblea_chat: i64 = std::env::var("ASAMBLEA_CHAT_ID")
        .expect("id not in chat")
        .parse()
        .expect("id not a number");
    let bot_name = std::env::var("BOT_NAME").unwrap();
    let bot = Bot::from_env();

    let listener = Polling::builder(bot.clone()).drop_pending_updates().build();

    log::info!("{} has awaken!", bot_name);

    teloxide::repl_with_listener(
        bot,
        move |bot: Bot, msg: Message| async move { handle_message(bot, msg).await },
        listener,
    )
    .await;
}
