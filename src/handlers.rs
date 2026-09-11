use teloxide::prelude::*;
use teloxide::types::MessageEntityKind;

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
