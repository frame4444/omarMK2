mod actions;
mod handlers;

use handlers::handle_text_message;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // carga el .env al entorno del proceso
    pretty_env_logger::init();
    log::info!("Arrancando el bot...");

    let omar = Bot::from_env();

    teloxide::repl(omar, handle_text_message).await;
}
