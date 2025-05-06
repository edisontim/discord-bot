use cainome::cairo_serde_derive::CairoSerde;
use serenity::{
    all::{CreateEmbed, CreateEmbedFooter, CreateMessage, Timestamp},
    model::id::ChannelId,
};

use crate::{
    constants::ETERNUM_URL,
    types::{DiscordMessage, DiscordMessageType},
};

use super::{Metadata, SubscriptionEvent};

#[derive(CairoSerde, Clone, Copy)]
pub struct ExplorerRaid {
    pub explorer_id: u32,
    pub structure_id: u32,
    pub success: bool,
    pub timestamp: u64,
}

impl SubscriptionEvent for ExplorerRaid {
    fn to_discord_message(&mut self, msg_type: DiscordMessageType) -> DiscordMessage {
        let footer = CreateEmbedFooter::new(ETERNUM_URL);
        let embed = CreateEmbed::new()
            .title(format!("TO BE IMPLEMENTED",))
            .color(poise::serenity_prelude::Color::RED)
            .footer(footer)
            .timestamp(Timestamp::now());

        let content = CreateMessage::new()
            .content("TO BE IMPLEMENTED!")
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

impl Metadata for ExplorerRaid {
    const EVENT_NAME: &'static str = "s1_eternum-ExplorerRaidEvent";
}
