use teloxide::types::{InlineKeyboardMarkup, ReplyParameters};

pub struct OmarPost {
    pub url: String,
    pub user: String,
    pub keyboard: InlineKeyboardMarkup,
    pub reply_parameter: ReplyParameters,
}
