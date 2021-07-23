pub type BotResult<T> = std::result::Result<T, BotError>;

pub enum BotError {
    SerenityError(serenity::Error),
    RequestError,
}

impl From<serenity::Error> for BotError {
    fn from(err: serenity::Error) -> Self {
        Self::SerenityError(err)
    }
}
