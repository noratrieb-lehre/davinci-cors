use crate::error::BotResult;
use dto::Event;

pub fn get_events(guild_id: u64, before: Option<u64>, after: Option<u64>) -> BotResult<Vec<Event>> {
    Ok(vec![])
}
