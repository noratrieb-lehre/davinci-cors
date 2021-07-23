use reqwest::Error;
use std::fmt::{Display, Formatter};

pub type BotResult<T> = std::result::Result<T, BotError>;

pub enum BotError {
    DiscordError(serenity::Error),
    CorsApiError(reqwest::Error),
    Other(&'static str),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BotError::DiscordError(err) => format!("Discord Error: {}", err),
                BotError::CorsApiError(err) => format!("CORS APi Error: {}", err),
                BotError::Other(msg) => format!("Other Error: {}", msg),
            }
        )
    }
}

impl From<serenity::Error> for BotError {
    fn from(err: serenity::Error) -> Self {
        Self::DiscordError(err)
    }
}

impl From<reqwest::Error> for BotError {
    fn from(err: Error) -> Self {
        Self::CorsApiError(err)
    }
}
