pub mod explorer_raid;
pub mod season_ended;
pub mod settle_realm;

use std::time::Duration;

use explorer_raid::ExplorerRaid;
use season_ended::SeasonEnded;
use settle_realm::SettleRealm;

use crate::types::{DiscordMessage, DiscordMessageType};

pub const UNKNOWN_USER: &str = "Unknown User";

pub trait SubscriptionEvent: Send {
    fn to_discord_message(&mut self, msg_type: DiscordMessageType) -> DiscordMessage;
    fn should_send_in_channel_if_no_user_found(&self) -> bool;
}

pub fn duration_to_string(duration: u64) -> String {
    let duration = Duration::from_secs(duration);
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub trait Metadata {
    const EVENT_NAME: &'static str;
}

pub const TORII_SUBSCRIPTION_MODELS: [&str; 3] = [
    SeasonEnded::EVENT_NAME,
    SettleRealm::EVENT_NAME,
    ExplorerRaid::EVENT_NAME,
];
