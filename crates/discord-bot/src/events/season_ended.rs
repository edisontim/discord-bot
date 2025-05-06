use cainome::cairo_serde_derive::CairoSerde;
use serenity::{
    all::{CreateEmbed, CreateEmbedFooter, CreateMessage, Timestamp},
    model::id::ChannelId,
};
use starknet_crypto::Felt;

use crate::{
    constants::ETERNUM_URL,
    types::{DiscordMessage, DiscordMessageType},
    utils::felt_to_string,
};

use super::{Metadata, SubscriptionEvent, UNKNOWN_USER};

#[derive(CairoSerde, Clone, Copy)]
pub struct SeasonEnded {
    pub winner_address: Felt,
    pub timestamp: u64,
}

impl SubscriptionEvent for SeasonEnded {
    fn to_discord_message(&mut self, msg_type: DiscordMessageType) -> DiscordMessage {
        let footer = CreateEmbedFooter::new(ETERNUM_URL);
        let embed = CreateEmbed::new()
            .title(format!(
                "{} has claimed the season",
                felt_to_string(&self.winner_address).unwrap_or(UNKNOWN_USER.to_string()),
            ))
            .color(poise::serenity_prelude::Color::RED)
            .footer(footer)
            .timestamp(Timestamp::now());

        let content = CreateMessage::new()
            .content("SEASON ENDED!")
            .embed(embed.clone());

        match msg_type {
            DiscordMessageType::ChannelMessage(channel_id) => DiscordMessage::ChannelMessage {
                channel_id: ChannelId::from(channel_id),
                content,
            },
            DiscordMessageType::DirectMessage(user_id) => {
                DiscordMessage::DirectMessage { user_id, content }
            }
        }
    }

    fn should_send_in_channel_if_no_user_found(&self) -> bool {
        true
    }
}

impl Metadata for SeasonEnded {
    const EVENT_NAME: &'static str = "s1_eternum-SeasonEnded";
}
