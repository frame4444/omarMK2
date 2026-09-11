use std::process::Stdio;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::types::MessageEntityKind;
use tokio::process::Command;

fn extract_link(msg: &Message) -> Option<String> {
    let text = msg.text()?;
    let entities = msg.entities()?;

    for entity in entities {
        let url = match &entity.kind {
            MessageEntityKind::Url => text
                .chars()
                .skip(entity.offset)
                .take(entity.length)
                .collect::<String>(),
            MessageEntityKind::TextLink { url } => url.to_string(),
            _ => continue,
        };

        if url.contains("tiktok.com") || url.contains("instagram.com") || url.contains("x.com") {
            return Some(url);
        }
    }
    None
}

pub async fn send_video(bot: Bot, msg: Message) -> ResponseResult<()> {
    log::info!("chat.id: {} | tipo: {:?}", msg.chat.id, msg.chat.kind);
    let Some(url) = extract_link(&msg) else {
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
            let group_id: i64 = std::env::var("GROUP_CHAT_ID")
                .expect("gcid not in .env")
                .parse()
                .expect("wrong gcid format");

            bot.send_video(ChatId(group_id), InputFile::file(&output_path))
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
