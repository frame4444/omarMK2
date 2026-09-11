mod actions;
mod handlers;
mod models;

use handlers::handle_message;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // carga el .env al entorno del proceso
    pretty_env_logger::init();
    log::info!("Omar has awaken");

    let bot = Bot::from_env();

    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        handle_message(bot, msg).await
    })
    .await;
}
