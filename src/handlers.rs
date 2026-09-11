use crate::actions::send_video::*;
use teloxide::prelude::*;

pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(post) = setup_info(&msg) {
        send_video(bot, msg, post).await?;
    }
    Ok(())
}
