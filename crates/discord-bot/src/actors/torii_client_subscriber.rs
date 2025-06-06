use cainome_cairo_serde::CairoSerde;
use dojo_types::schema::Ty;
use serenity::futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

use starknet_crypto::Felt;
use torii_grpc::types::schema::Entity;

use torii_grpc::types::{EntityKeysClause, KeysClause, PatternMatching};

use crate::events::explorer_raid::ExplorerRaid;
use crate::events::season_ended::SeasonEnded;
use crate::events::{Metadata, TORII_SUBSCRIPTION_MODELS};
use crate::{
    events::settle_realm::SettleRealm,
    types::{Config, Event},
};

pub struct ToriiClientSubscriber {
    client: torii_client::client::Client,
    processed_event_sender: mpsc::Sender<Event>,
}

impl ToriiClientSubscriber {
    pub async fn new(
        config: Config,
        processed_event_sender: mpsc::Sender<Event>,
    ) -> eyre::Result<Self> {
        let client = torii_client::client::Client::new(
            config.torii_url.clone(),
            config.torii_relay_url.clone(),
            Felt::from_hex_unchecked(&config.world_address.clone()),
        )
        .await?;

        Ok(Self {
            client,
            processed_event_sender,
        })
    }

    pub async fn subscribe(self) {
        tracing::info!("Setting up Torii client");
        let mut tries = 0;
        let max_num_tries = 200;

        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        while tries < max_num_tries {
            let rcv = self
                .client
                .on_event_message_updated(vec![EntityKeysClause::Keys(KeysClause {
                    keys: vec![],
                    pattern_matching: PatternMatching::VariableLen,
                    models: TORII_SUBSCRIPTION_MODELS
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                })])
                .await;

            match rcv {
                Ok(mut rcv) => {
                    backoff = Duration::from_secs(1);

                    loop {
                        match rcv.next().await {
                            Some(result) => {
                                if let Ok((_, entity)) = result {
                                    self.treat_received_torii_event(entity).await;
                                } else {
                                    tracing::warn!(
                                        "Received invalid data from torii {}, reconnecting",
                                        result.err().unwrap()
                                    );
                                    break;
                                }
                            }
                            None => {
                                tracing::warn!("Stream returned an error, reconnecting");
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Subscription failed: {:?}", e);
                    tries += 1;
                }
            }

            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }

        tracing::error!("Torii client disconnected, reached max number of tries");
    }

    async fn treat_received_torii_event(&self, entity: Entity) {
        for model in entity.models {
            let ty = Ty::Struct(model.clone());

            let felts = ty.serialize().unwrap();

            tracing::info!("Received event: {}", model.name);
            let event = match model.name.as_str() {
                SettleRealm::EVENT_NAME => {
                    let event = SettleRealm::cairo_deserialize(&felts, 0).unwrap();
                    Event {
                        event: Box::new(event),
                        identifier: event.owner_address,
                    }
                }
                SeasonEnded::EVENT_NAME => 	{
                    let event = SeasonEnded::cairo_deserialize(&felts, 0).unwrap();
                    Event {
                        event: Box::new(event),
                        identifier: event.winner_address,
                    }
                }
                ExplorerRaid::EVENT_NAME => {
                    let event = ExplorerRaid::cairo_deserialize(&felts, 0).unwrap();
                    Event {
                        event: Box::new(event),
                        identifier: event.explorer_id.into(),
                    }
                }
                _ => {
                    tracing::warn!("Unknown model name: {}", model.name);
                    continue;
                }
            };
            self.processed_event_sender.send(event).await.unwrap();
        }
    }
}
