use crate::models::post::OmarPost;

use std::process::Stdio;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, MessageEntityKind};
use tokio::process::Command;

pub async fn send_video(bot: Bot, msg: Message, post: OmarPost) -> ResponseResult<()> {
    log::info!(
        "chat.id: {} | type: {:?} | url: {}",
        msg.chat.id,
        msg.chat.kind,
        post.url
    );

    let output_path = format!("/tmp/{}.mp4", msg.id);

    let status = Command::new("yt-dlp")
        .arg("-o")
        .arg(&output_path)
        .arg("-f")
        .arg("mp4")
        .arg(&post.url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;

    match status {
        Ok(s) if s.success() => {
            bot.send_video(msg.chat.id, InputFile::file(&output_path))
                .caption(post.user)
                .reply_markup(post.keyboard)
                .await?;
            let _ = tokio::fs::remove_file(&output_path).await;
        }
        _ => {
            log::warn!("download failed for {}", post.url);
            bot.send_message(msg.chat.id, "Nigga fuck you").await?;
        }
    }

    Ok(())
}

pub fn setup_info(msg: &Message) -> Option<OmarPost> {
    let url = extract_link(msg)?;

    let username = match msg.from.as_ref() {
        Some(user) => match &user.username {
            Some(username) => format!("@{}", username),
            None => user.first_name.clone(),
        },
        None => "noname".to_string(),
    };
    let user = username;

    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "Ver comentarios",
        url::Url::parse(&url).ok()?,
    )]]);

    Some(OmarPost {
        url,
        user,
        keyboard,
    })
}

pub fn extract_link(msg: &Message) -> Option<String> {
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
