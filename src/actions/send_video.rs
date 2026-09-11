use crate::handlers;

use std::process::Stdio;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use tokio::process::Command;

pub async fn send_video(bot: Bot, msg: Message) -> ResponseResult<()> {
    log::info!("chat.id: {} | tipo: {:?}", msg.chat.id, msg.chat.kind);
    let Some(url) = handlers::extract_link(&msg) else {
        return Ok(()); //msg has link in it 
    };

    let output_path = format!("/tmp/{}.mp4", msg.id);

    let status = Command::new("yt-dlp")
        .arg("-o")
        .arg(&output_path)
        .arg("-f")
        .arg("mp4")
        .arg(&url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;

    match status {
        Ok(s) if s.success() => {
            bot.send_video(msg.chat.id, InputFile::file(&output_path))
                .await?;
            let _ = tokio::fs::remove_file(&output_path).await;
        }
        _ => {
            log::warn!("download failed for {}", url);
            bot.send_message(msg.chat.id, "Nigga fuck you").await?;
        }
    }

    Ok(())
}
