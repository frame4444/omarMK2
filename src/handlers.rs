use teloxide::prelude::*;
pub async fn handle_text_message(omar: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        log::info!("recieved: {}", text);
    }
    omar.send_message(msg.chat.id, "pong").await?;
    Ok(())
}
