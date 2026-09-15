use teloxide::types::{InlineKeyboardMarkup, ReplyParameters};

pub struct OmarPost {
    pub url: String,
    pub user: String,
    pub keyboard: InlineKeyboardMarkup,
    pub reply_to_sender: ReplyParameters,
    pub reply_to_sender_reply: ReplyParameters,
}
