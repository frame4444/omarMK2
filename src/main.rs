mod handlers;

use handlers::send_video;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // carga el .env al entorno del proceso
    pretty_env_logger::init();
    log::info!("Omar has awaken");

    let bot = Bot::from_env();

    teloxide::repl(bot, send_video).await;
}
